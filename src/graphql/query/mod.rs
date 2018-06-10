use ::graphql::Context;

mod lydie_suelle;

pub struct Query;
use self::lydie_suelle::LydieSuelle;

graphql_object!(Query: Context |&self| {
    field api_version() -> &str as
    "현재 API 버전입니다."
    {
        env!("CARGO_PKG_VERSION")
    }

    field source_code_url() -> &str as
    "서버의 소스 코드가 있는 URL입니다."
    {
        option_env!("SOURCE_CODE_URL")
            .unwrap_or("https://github.com/tirr-c/plachta.git")
    }

    field lydie_suelle() -> LydieSuelle as
    "<리디&수르의 아틀리에> 정보를 쿼리합니다."
    {
        LydieSuelle
    }
});
