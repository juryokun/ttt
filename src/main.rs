fn main() {
    println!("Hello, world!");
}
// curl -X POST 'https://api.notion.com/v1/databases/${database_id}/query' \ 812ms  2021年10月09日 17時27分57秒
//            -H 'Authorization: Bearer '"${api_key}"'' \
//            -H 'Notion-Version: 2021-08-16' \
//            -H "Content-Type: application/json" \
//            --data '{
//            "filter": {
//                "and": [
//                    {
//                        "property": "Today flag",
//                        "checkbox": {
//                          "equals": true
//                        }
//                    },
//                    {
//                        "property": "Done flag",
//                        "checkbox": {
//                          "equals": false
//                        }
//                    }
//                ]
//            }
//        }'
