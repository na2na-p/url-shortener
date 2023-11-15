use juniper::Context as JuniperContext;

// GraphQLのコンテキスト型
pub struct Context {
    // ここにコンテキストに含めるフィールドを追加
    // 例: データベース接続、設定情報、認証情報など
}

impl JuniperContext for Context {}
