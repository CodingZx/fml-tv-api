use std::sync::RwLock;
use crate::common::util::rsa_util::{RSAError, RSAUtil, RSAPair};

// consts CLIENT_PRIVATE_KEY: &str = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCnD3Pc56RBsU/MiCF8JUwSfkWcqy8xZdXGvGVSt01Ark3EpKY6bHSdPDt6Zwmj6T7QQNuyGVr6Mq0ZJNLCec5W5GkY+VURB80A34JDuU9g31mGaXWQlpGpnvBaz3/sGTidHYMf8VWuIdChZsqZY+PJNfpO2ANhL0puGc7+DXCi3FhRPy8a45amgQsr870A1fsrMV31XLQUVF/zWq//TSiST2Xa1VG9IZoEgoYOKbh957MaIM8cgAfnZgcwZCaXzGFwvljK51zORmyhvvXm3AC39enfrbEv5H8hAFCg33OtwB16akzYL/RpesHMjnNLNyeJNciJknPX/1Igwa0LdsSnAgMBAAECggEAWQ4rvX7eWQake2u2Le/T9afKhBHdPoACzcZwXP/J9sj0O/dphYt7PrZ1HnL++JnGGk34z6Akl2ucSuMOVw2HqVppjB6d8k4uQmDjmCaY4hMuLjDZsxIQ5FMN3drvXnSzcU3iMIz+F61/AgtBxTMyonhbztvoMU58ZuTrKxMPEEF5VJ2qXmZnhw7xGtwR4rI5OukiX2vljG3mDMLw//lD22rRNDwsz2wkydpCPV3wjir+pZnDmWKagsHDmGZsY1y681OYEe5+yxDBnBRjPBIQkj7/aZJ3naKqM+H8ugW7rIaQWRBBrMoqbg0YAo5EppBYjY1n9ZuDilPM12N/NUP8QQKBgQDHun61/WKbV8W8KldjPACVD2t+dpkYbYB0BtKhcvUcyM6NI7pxLj83eWJFFHm0j71ctmcfKxbPULdIvnExi/z4DWgm07aNlCW3WLk1jRFyVBQSd+KpFdJeS98y73AO7LPqO4ehbtVd8GzAF2cZcF3AlA+COJg/eYUq7PrKdfT0HQKBgQDWIMERF0jHYzOoRgrvQ0AAp9xnUStU5KXxtT6A1rfOM0QjsvZKKu19/jT5z8WacLB8mzyjjQFuty+qJW7oE+RLFI9ihbXB95h/BmJv3C9o5kJv/jO4jT7Mi6VckIfMB1GDgE11NVQWJhvbBSBDfBSOxumAn2Mv7fzEk9rioxV4kwKBgFVcxc8ubriJ+P8E8bh96pnKr1yrf59LHb15NQi/wzb8NBtqi8LXyzkAbhmsOKANVxeK9eQCXItaqhszGdndRM8Xm84Mald0W/JSv0W99xjwCmwiRQOTSgf3UXvJjhEy5WpU6xD598NggHpRBkV/GvK11TMI0Tk/zMSd6Eojw/+ZAoGAZs1C49mx70gJPPijt0sqJyZwAopNq2w631iMsX5ksHKcfCcnZ2REsQCinmzKCeOgV7KP4lWIIoeAMsfV/0XNjN5KGJrpMD6W0CVYjSvA9zPVIb+BRVFKnRlOQqqBB8tbry6iAWug+yFArl4/pyajGAkXgdED5bb0a8wwgi6Z2FECgYEApBMrQ3rssRm3JByhMYEf5Ij8UGYzXEghUTAiII4MvpneAHckRYsgRpkmkMUlFpve13UUTVgm7ofljKt9C6pZKe+bTl6pGVAhGWOdqoq8bAugPpH/4WrEMvAX/srsCqO36Odcmzd95YACgkuHax0pwNM4G7GIiazr+H6jzeuBJoU=";
const CLIENT_PUBLIC_KEY: &str = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApw9z3OekQbFPzIghfCVMEn5FnKsvMWXVxrxlUrdNQK5NxKSmOmx0nTw7emcJo+k+0EDbshla+jKtGSTSwnnOVuRpGPlVEQfNAN+CQ7lPYN9Zhml1kJaRqZ7wWs9/7Bk4nR2DH/FVriHQoWbKmWPjyTX6TtgDYS9KbhnO/g1wotxYUT8vGuOWpoELK/O9ANX7KzFd9Vy0FFRf81qv/00okk9l2tVRvSGaBIKGDim4feezGiDPHIAH52YHMGQml8xhcL5YyudczkZsob715twAt/Xp362xL+R/IQBQoN9zrcAdempM2C/0aXrBzI5zSzcniTXIiZJz1/9SIMGtC3bEpwIDAQAB";

const SERVER_PRIVATE_KEY: &str = "MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDRQMVp06D1urMLvLLZ993776LcUL54Qao5cYkKn1lSnAQHO5tHyzR3mxgP8wXQMzjRzxv+cVwJmI8PR4Qw6eFGzX30HgxQoC1+aC0i3CcAoiYYVgkoNJAdkzrxakFLhFYb9zNQSkSLUOT99kGZOu+GEGB1Bp8Coghf/Ow+rvte0ZfN3T9ywh+8RPGlv4XUFlWRmdZPvq+RkXsE9VHrmMIuBKWCSxpa4gf1ggeenWbnmIYOZZq+lAT7ZN22UJrpPb/nBF2LKQCfhaehD35bZ77WPsDquKdz8nFHdSFhK/GNaoWIf5/BDAYyPFuhL9MHACj/j7QVM/nGsgK81Ofrghd7AgMBAAECggEAa8UZJ2TYXFZ4ik34Kyxuhqen0CJBxrvDCOzBcNga7+HsbTE0yygL8KHPJb+7Obx3wg8R0tzdUw/gdjUWDv8ViBqjiQvEue2VIKpUvMR3zeq1Bgmhk7RmmOTQEa/wywzwMF0Db5GgdcQG/AlNRbbFz901YHA1iuyatZ0Uq0ABtJX89C5+t9QVO7DxgxWrVfd7vFuzrOQzK+dQyDU3qyXGFSIGz4B6vsdVbvQ21LVpDs4KZ29TiRZs9bIQtNinFAWpqpZR2BWkQcT+LWtehUpddfPqV1SqkcjoaVAX1+GYeCBnaFXFW/AarUsMVthyq/VCMO+UVAXTQ+FXpIvCZrLm4QKBgQD1NMkAYI8klT/3yQ2qv/VX1KshG9XLlYUQofXKsKn6PWloKewHIC5C+6qzFmdc+nYQeGUg/UQ8stUKBEPAaMAakXc7Dapqm0XU2yjf8HWgMpfC55ZAPnVVH7zqNK900+5W08h0WrxRA2tK1p+Gzic8vx2aHh9wAoTdRlWoToJDMQKBgQDadtTkykq6SpjXlHNum5MOj8k/Zzo75VY3Gpa91++0X16pnh+6NyzBhda1vFpPRwaGI6rasnv2Nt3xwOuzw0BrHwTOtWlRAnUczWXVkyzFyMiBhpqvyg7eHLfdji3VSn6LdXl4y1GAKBN8Zq9SciQq68v18RZZQg1Ho7hvpHuiawKBgQCK+k0GirP2/yTPc0PsyHntOxt3W4h0qB1QTQu7wx/ENnR81m3xp/qwemXItWUGXOr38NUYupOnd5Cp6brg3j4rkNEsRlWVsMYGm+Tx8B8rGiHMFWiipwPnqGJxuETOCaSnz8dos1jS7gYs17J5PHUeyA+mH3agDL1zkP7RsxW/sQKBgAKFJGePPdVwTtsNx1cbTFL6raOhmTZfquLv15VRDF7USs9CF7OaN1X0KUnnZlNLynN5rdrf/DYW4/CJ/S4RroiNzwxjY1ef46H1bJtOG1IcwtNbriUg+5LsWeKgZT70oQkSzdfq4IHN9IuW06br0ib8mVm48j0NWgHdjZ8cEqEJAoGBAJaFaOlJt9qArJZ8ccxvvRqeokWJrArHG8BDWG1/Wg9V2M1EaNSbbHIzt9qayzJNhWLUWGPwux4xvoW0kCGhUtHqfS+nnpeO/TdpINaoZttlMdzKwF1AAggOaweUnOkPBSEVrPhgWpXtDp1pvNX+gT2j4F435/Aja7eXDXRXOYVT";
// consts SERVER_PUBLIC_KEY: &str = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0UDFadOg9bqzC7yy2ffd+++i3FC+eEGqOXGJCp9ZUpwEBzubR8s0d5sYD/MF0DM40c8b/nFcCZiPD0eEMOnhRs199B4MUKAtfmgtItwnAKImGFYJKDSQHZM68WpBS4RWG/czUEpEi1Dk/fZBmTrvhhBgdQafAqIIX/zsPq77XtGXzd0/csIfvETxpb+F1BZVkZnWT76vkZF7BPVR65jCLgSlgksaWuIH9YIHnp1m55iGDmWavpQE+2TdtlCa6T2/5wRdiykAn4WnoQ9+W2e+1j7A6rinc/JxR3UhYSvxjWqFiH+fwQwGMjxboS/TBwAo/4+0FTP5xrICvNTn64IXewIDAQAB";


static SERVER_ENCRYPT_RSA: RwLock<Option<RSAPair>> = RwLock::new(None);
static CLIENT_DECRYPT_RSA: RwLock<Option<RSAPair>> = RwLock::new(None);

pub struct VueEncryptUtil;

impl VueEncryptUtil {

    /// 服务端返回数据加密
    pub fn server_encrypt(value: &str) -> Result<String, RSAError> {
        loop {
            let read = SERVER_ENCRYPT_RSA.read();
            if read.is_ok() {
                let guard = read.unwrap();
                if let Some(rsa) = &*guard {
                    return rsa.encrypt(value)
                }
            }

            let write = SERVER_ENCRYPT_RSA.write();
            if write.is_ok() {
                let mut guard = write.unwrap();
                if guard.is_some() {
                    continue;
                }
                let rsa = RSAUtil::from_keypair(CLIENT_PUBLIC_KEY, "")?;
                *guard = Some(rsa);
                continue;
            }
        }
    }

    /// 客户端请求数据解密
    pub fn client_decrypt(value: &str) -> Result<String, RSAError> {
        loop {
            let read = CLIENT_DECRYPT_RSA.read();
            if read.is_ok() {
                let guard = read.unwrap();
                if let Some(rsa) = &*guard {
                    return rsa.decrypt(value)
                }
            }

            let write = CLIENT_DECRYPT_RSA.write();
            if write.is_ok() {
                let mut guard = write.unwrap();
                if guard.is_some() {
                    continue;
                }
                let rsa = RSAUtil::from_keypair("", SERVER_PRIVATE_KEY)?;
                *guard = Some(rsa);
                continue;
            }
        }
    }
}
