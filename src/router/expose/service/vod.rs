use crate::common::error::ServerError::BusinessError;
use crate::common::model::{ExposePager, Page};
use crate::common::state::AppState;
use crate::common::{consts, ServerResult};
use crate::database::dao::collect_site::CollectSiteDao;
use crate::database::dao::collect_vod::CollectVodDao;
use crate::database::dao::collect_vod_episode::CollectVodEpiDao;
use crate::database::dao::sys_config::SysConfigDao;
use crate::database::dao::tv_group::TvGroupDao;
use crate::database::dao::tv_type::TvTypeDao;
use crate::database::dao::tv_vod::TvVodDao;
use crate::database::dao::tv_vod_pic::TvVodPicDao;
use crate::database::model::sys_config::EasyBangumiConfig;
use crate::database::model::tv_vod::{TvVodColumn, TvVodModel};
use crate::router::expose::service::JS_CONTENT;
use crate::router::expose::vo::vod::{JsQuery, TypeGroupReq, TypeListResp, VodDetailReq, VodDetailResp, VodEpisodeLineResp, VodEpisodePlayReq, VodEpisodePlayResp, VodEpisodeResp, VodListReq, VodListResp, VodSearchReq};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::ArrayType;
use sea_orm::{ColumnTrait, Condition};
use std::collections::HashMap;
use uuid::Uuid;

pub struct VodService<'d> {
    state: &'d AppState
}

impl<'d> VodService<'d> {
    
    pub fn new(state: &'d AppState) -> Self {
        Self { state }
    }

    pub async fn js_content(&self, param: JsQuery) -> ServerResult<String> {
        let conf_key = consts::conf_keys::easy_bangumi_conf_key();
        let config = SysConfigDao::new(&self.state.db).get_json_conf::<EasyBangumiConfig>(&conf_key).await?.unwrap_or_default();
        if !config.js_enable {
            return Err(BusinessError("JS接口未启用"))
        }

        let param_types = param.types.unwrap_or_default();
        let show_groups = if param_types.is_empty() {
            // 默认显示全部
            TvGroupDao::new(&self.state.db).find_all().await?
        } else {
            let group_dao = TvGroupDao::new(&self.state.db);
            let mut groups = Vec::new();
            for type_name in param_types {
                if let Some(v) = group_dao.find_by_name(&type_name).await? {
                    groups.push(v);
                }
            }
            groups
        };

        let main_tab_ph = "$$$MAIN_TAB$$$";
        let mut main_tabs = String::new();
        main_tabs.push_str("    var res = new ArrayList();");
        for show_group in show_groups {
            let title = show_group.name;
            if show_group.types.0.len() > 1 {
                // 有次级Tab
                main_tabs.push_str(&format!(r#"    res.add(new MainTab("{title}", MainTab.MAIN_TAB_GROUP)); "#));
                main_tabs.push_str("\n");
            } else {
                // 只有一个, 不需要次级标题
                main_tabs.push_str(&format!(r#"    res.add(new MainTab("{title}", MainTab.MAIN_TAB_WITH_COVER)); "#));
                main_tabs.push_str("\n");
            }
        }
        main_tabs.push_str("    return res;");


        let req_url_ph = "$$$REQ_URL$$$";
        let req_url = config.request_url;
        if req_url.is_empty() {
            return Err(BusinessError("接口请求地址未配置"))
        }

        let js_key_ph = "$$$JS_KEY$$$";
        let js_key = config.js_key;

        let js_label_ph = "$$$JS_LABEL$$$";
        let js_label = config.js_label;

        let js_lib_ver_ph = "$$$JS_LIB_VER$$$";
        let js_lib_ver = config.js_lib_ver;

        let js_cover_ph = "$$$JS_COVER$$$";
        let js_cover = config.js_cover;

        let js_ver_code_ph = "$$$JS_VER_CODE$$$";
        let js_ver_code = config.js_version_code;

        let js_ver_name_ph = "$$$JS_VER_NAME$$$";
        let js_ver_name = config.js_version_name;

        let req_token_ph = "$$$REQ_TOKEN$$$";
        let req_token = config.request_token;

        let content = JS_CONTENT
            .replace(js_key_ph, &js_key)
            .replace(js_label_ph, &js_label)
            .replace(js_cover_ph, &js_cover)
            .replace(js_lib_ver_ph, &js_lib_ver)
            .replace(js_ver_name_ph, &js_ver_name)
            .replace(js_ver_code_ph, &js_ver_code)
            .replace(main_tab_ph, &main_tabs)
            .replace(req_url_ph, &req_url)
            .replace(req_token_ph, &req_token);

        Ok(content.to_string())
    }

    pub async fn types(&self, param: TypeGroupReq) -> ServerResult<Vec<TypeListResp>> {
        let name = param.name;
        let bind_types = match TvGroupDao::new(&self.state.db).find_by_name(&name).await? {
            Some(v) => v.types.0,
            None => return Ok(vec![])
        };
        if bind_types.is_empty() {
            return Ok(vec![])
        }
        let types = TvTypeDao::new(&self.state.db).find_by_ids(bind_types).await?;
        let mut resp = Vec::new();
        for typ in types {
            resp.push(TypeListResp::new(typ));
        }
        Ok(resp)
    }

    pub async fn vod_list(&self, param: VodListReq) -> ServerResult<ExposePager<VodListResp>> {
        let type_id = match Uuid::parse_str(&param.typ) {
            Ok(v) => v,
            Err(_) => {
                // 谁会把uuid当Group的名称.....
                if let Some(group) = TvGroupDao::new(&self.state.db).find_by_name(&param.typ).await? {
                    Vec::from_iter(group.types.0)[0]
                } else {
                    return Ok(ExposePager::new(vec![], 0))
                }
            },
        };
        let typ_op = TvTypeDao::new(&self.state.db).find_by_id(type_id).await?;
        if typ_op.is_none() {
            return Ok(ExposePager::new(vec![], 0))
        }

        // 绑定的类型
        let array_literal = format!(r#" "tv_type" ?| ARRAY['{}'] "#, type_id);
        let conditions = Condition::all()
            .add(Expr::cust(array_literal))
            .add(TvVodColumn::Show.eq(true));

        let page = Page::from(param.page, param.size);
        let (records, count) = TvVodDao::new(&self.state.db).find_page_list(page, conditions).await?;
        let result = self.build_vod_list(records).await?;
        Ok(ExposePager::new(result, count))
    }

    pub async fn vod_detail(&self, param: VodDetailReq) -> ServerResult<VodDetailResp> {
        let vod = TvVodDao::new(&self.state.db).find_by_id(param.id).await?.ok_or(BusinessError("ID错误, 数据不存在"))?;
        if !vod.show {
            return Err(BusinessError("ID错误, 数据不存在"))
        }

        let pic = TvVodPicDao::new(&self.state.db)
            .find_one_by_vod_id(vod.id)
            .await?
            .map(|t|t.pic)
            .unwrap_or("".to_string());

        let detail = VodDetailResp {
            id: vod.id,
            name: vod.name.clone(),
            cover: pic,
            genre: "".to_string(),
            intro: vod.vod_blurb.clone(),
            description: vod.vod_content.clone(),
            lines: self.vod_lines(vod).await?,
        };
        Ok(detail)
    }

    pub async fn vod_search(&self, param: VodSearchReq) -> ServerResult<ExposePager<VodListResp>> {
        let page = Page::from(param.page, param.size);
        let mut conditions = Condition::all();

        let keyword = param.keyword.trim();
        if keyword.chars().count() < 3 {
            let expr = r#" split_12("clear_name") @> $1 and "show" = $2 "#;

            let values = vec![
                sea_orm::Value::Array(ArrayType::String, Some(Box::new(vec![sea_orm::Value::String(Some(keyword.to_string()))]))),
                sea_orm::Value::Bool(Some(true))
            ];
            conditions = conditions.add(Expr::cust_with_values(expr, values));
        } else {
            conditions = conditions.add(TvVodColumn::ClearName.contains(keyword));
            conditions = conditions.add(TvVodColumn::Show.eq(true));
        }

        let (records, count) = TvVodDao::new(&self.state.db).find_page_list(page, conditions).await?;
        let result = self.build_vod_list(records).await?;
        Ok(ExposePager::new(result, count))
    }

    async fn build_vod_list(&self, list: Vec<TvVodModel>) -> ServerResult<Vec<VodListResp>> {
        let mut result = Vec::with_capacity(list.len());
        for vod in list {
            let pic = TvVodPicDao::new(&self.state.db)
                .find_one_by_vod_id(vod.id)
                .await?
                .map(|t|t.pic)
                .unwrap_or("".to_string());
            result.push(VodListResp::from(&vod, pic))
        }
        Ok(result)
    }

    pub async fn play_url(&self, param: VodEpisodePlayReq) -> ServerResult<VodEpisodePlayResp> {
        let episode = CollectVodEpiDao::new(&self.state.db).find_by_id(param.id).await?;

        let url = episode.map(|v| v.url).unwrap_or("".to_string());
        Ok(VodEpisodePlayResp {
            url
        })
    }

    async fn vod_lines(&self, vod: TvVodModel) -> ServerResult<Vec<VodEpisodeLineResp>> {
        let collect_vod_dao = CollectVodDao::new(&self.state.db);
        let collect_site_dao = CollectSiteDao::new(&self.state.db);
        let collect_epi_dao = CollectVodEpiDao::new(&self.state.db);
        let mut lines = Vec::new();
        let mut unknow_site_idx = 0;
        for collect_vod_id in vod.collect_vod.0 {
            let site_id = collect_vod_dao.find_site_by_id(collect_vod_id).await?;
            let site = collect_site_dao.find_by_id(site_id).await?;
            let episodes = collect_epi_dao.find_by_vod_id(collect_vod_id).await?;

            let site_name = if let Some(v) = site {
                v.name
            } else {
                unknow_site_idx = unknow_site_idx + 1;
                format!("未知站点{unknow_site_idx}")
            };

            let mut order_num = 1i32;
            let mut vod_lines = HashMap::<String, Vec<VodEpisodeResp>>::new();
            for episode in episodes {
                let line = episode.line;
                let resp = VodEpisodeResp {
                    id: episode.id,
                    name: episode.name,
                    order: order_num,
                };
                order_num = order_num + 1;
                vod_lines.entry(line).or_insert(vec![]).push(resp);
            }

            for (line_name, episodes) in vod_lines {
                let line = format!("{site_name}-{line_name}");
                lines.push(VodEpisodeLineResp { line, episodes })
            }
        }
        Ok(lines)
    }
}