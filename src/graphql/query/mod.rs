use ::graphql::Context;

mod lydie_suelle;

pub struct Query;
use self::lydie_suelle::LydieSuelle;

graphql_object!(Query: Context |&self| {
    field api_version() -> &str as
    "현재 API 버전입니다."
    {
        "0.1"
    }

    field lydie_suelle() -> LydieSuelle as
    "<리디&수르의 아틀리에> 정보를 쿼리합니다."
    {
        LydieSuelle
    }
});
