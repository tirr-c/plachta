use ::graphql::Context;

mod lydie_suelle;

pub struct Mutation;
use self::lydie_suelle::LydieSuelle;

#[derive(GraphQLEnum, Copy, Clone, PartialEq, Eq, Debug)]
enum ModifyCategoryOp {
    Add,
    Remove,
}

graphql_object!(Mutation: Context |&self| {
    field lydie_suelle() -> LydieSuelle as
    "<리디&수르의 아틀리에> 정보를 변경합니다."
    {
        LydieSuelle
    }
});
