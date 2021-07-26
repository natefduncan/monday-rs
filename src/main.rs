use graphql_client::{GraphQLQuery, Response};
mod monday;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/board.graphql",
    response_derives = "Debug,Serialize"
)]
struct BoardView;

fn main() {
    let client = monday::get_client().expect("Could not get client.");
    let variables = board_view::Variables {
        board_id: Some(vec![Some(1393475156)]),
    };
    let res: Response<board_view::ResponseData> =
        monday::query::<BoardView>(&client, variables).expect("Could not execute query.");
    println!("{:#?}", res);
}
