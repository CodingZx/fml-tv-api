// @key $$$JS_KEY$$$
// @label $$$JS_LABEL$$$
// @versionName $$$JS_VER_NAME$$$
// @versionCode $$$JS_VER_CODE$$$
// @libVersion $$$JS_LIB_VER$$$
// @cover $$$JS_COVER$$$

// 注入对象
var okHttpHelper = Inject_OkhttpHelper;
var stringHelper = Inject_StringHelper;

// 静态对象
SourceUtils;
OkhttpUtils;


// 获取主Tab
function PageComponent_getMainTabs() {
    $$$MAIN_TAB$$$
}

// 获取二级 Tab
function PageComponent_getSubTabs(mainTab) {
    var url = getReqUrl("/expose/types");
    var param = {
        name: mainTab.label,
    }
    var tabs = sendRequest(url, param)

    var res = new ArrayList();
    for(var i=0;i<tabs.length;i++) {
        var obj = tabs[i];
        res.add(new SubTab(obj.name, true, obj.id));
    }
    return res;
}

/**
 * 获取首页番剧，分页加载，支持异步操作，可以发起网络请求
 * @param mainTab
 * @param subTab
 * @param key 页码，从 0 开始
 * @return Pair<Int?, List<CartoonCover>> 下一页页码（为空代表当前最后一页）
 */
function PageComponent_getContent(mainTab, subTab, key) {
    var nowPage = key + 1; // 页数从1开始
    var size = 20;

    var param = {
        page: nowPage,
        size: size,
    };
    if(subTab == null) {
        // 如果没有二级Tab, 则用主Tab的名称作为过滤, 接口已处理
        param.typ = mainTab.label
    } else {
        param.typ = subTab.ext
    }

    // 请求接口
    var url = getReqUrl("/expose/list");
    var resp = sendRequest(url, param)

    var cartoonList = new ArrayList();
    for(var i=0;i<resp.data.length;i++) {
        var obj = resp.data[i]
        cartoonList.add(makeCartoonCover({
            id: obj.id,
            title: obj.name,
            url: "",
            intro: "",
            cover: obj.cover,
        }))
    }
    var nextKey = null;
    var totalPage = resp.count % size == 0 ? (resp.count / size) : (resp.count / size + 1)
    if(nowPage < totalPage) {
        nextKey = nowPage + 1
    }
    return new Pair(nextKey, cartoonList);
}

/**
 * 根据关键字搜索番剧
 * @param page 页码，从 1 开始，后续为返回值的第一个分量
 * @param keyword 搜索关键字
 * @return Pair<Int?, List<CartoonCover>> 首个分量为空代表当前是最后一页
 */
function SearchComponent_search(page, keyword) {
    var nowPage = page <= 1 ? 1 : page
    var size = 20
    var param = {
        page: nowPage,
        size: size,
        keyword: keyword,
    };

    // 请求接口
    var url = getReqUrl("/expose/search");
    var resp = sendRequest(url, param)

    var cartoonList = new ArrayList();
    for(var i=0;i<resp.data.length;i++) {
        var obj = resp.data[i]
        cartoonList.add(makeCartoonCover({
            id: obj.id,
            title: obj.name,
            url: "",
            intro: "",
            cover: obj.cover,
        }))
    }

    var nextPage = null;
    var totalPage = resp.count % size == 0 ? (resp.count / size) : (resp.count / size + 1)
    if(nowPage < totalPage) {
        nextPage = nowPage + 1
    }
    return new Pair(nextPage, cartoonList);
}


/**
 * 获取播放线路和详细信息
 * @param summary 番剧摘要 CartoonSummary，id 为首页或搜索中返回的 CartoonCover 的数据
 * @return Pair<Cartoon, List<PlayLine>>
 */
function DetailedComponent_getDetailed(summary) {
    var param = {
        id: summary.id,
    };

    // 请求接口
    var url = getReqUrl("/expose/detail");
    var resp = sendRequest(url, param)

    var cartoon = makeCartoon({
        id: resp.id,
        url: "",
        source: summary.source,
        title: resp.name,
        genreList: resp.genre,
        cover: resp.cover,
        intro: resp.intro,
        description: resp.description,
        status: Cartoon.STATUS_UNKNOWN,
        updateStrategy: Cartoon.UPDATE_STRATEGY_ALWAYS,
        isUpdate: false
    });

    var playLines = new ArrayList();
    for(var i=0;i<resp.lines.length;i++) {
        var line = resp.lines[i];

        var episodes = new ArrayList();
        for(var j=0;j<line.episodes.length;j++) {
            var epi = line.episodes[j]
            episodes.add(new Episode(
                epi.id,
                epi.name,
                epi.order
            ));
        }

        playLines.add(new PlayLine(
            line.line + '-' + 'i',
            line.line,
            episodes
        ));
    }
    return new Pair(cartoon, playLines);
}

/**
 * 获取播放地址
 * @param summary 番剧摘要 CartoonSummary，id 为首页或搜索中返回的 CartoonCover 的数据
 * @param playLine 播放列表  getDetailed 返回
 * @param episode 当前集  getDetailed 返回
 * @return PlayerInfo 支持 m3u8 以及 普通 mp4，支持添加自定义请求 Header
 */
function PlayComponent_getPlayInfo(summary, playLine, episode) {
    var param = {
        id: episode.id
    }
    // 请求接口
    var url = getReqUrl("/expose/play/url");
    var resp = sendRequest(url, param)

    var playUrl = resp.url

    var type = PlayerInfo.DECODE_TYPE_OTHER;
    if (playUrl.indexOf(".m3u8") > -1) {
        type = PlayerInfo.DECODE_TYPE_HLS;
    }
    return new PlayerInfo(type, playUrl)
}


function sendRequest(url, param) {
    param.reqToken = "$$$REQ_TOKEN$$$"

    var client = okHttpHelper.client
    var requestBody = RequestBody.create(
        MediaType.get('application/json; charset=utf-8'),
        JSON.stringify(param));
    var request = new Request.Builder()
        .url(url)
        .post(requestBody)
        .build();
    var response = client.newCall(request).execute();
    var responseData = JSON.parse(response.body().string());
    if(responseData.code == 200) {
        return responseData.data
    }
    return {}
}

function getReqUrl(path) {
    var url = "$$$REQ_URL$$$";
    return SourceUtils.urlParser(url, path);
}