
use library_blockchain::*;

fn main() {
    println!("Hello, world!");

    library_blockchain::blockchain_executive::main();
}



















// #[wasm_bindgen]
// pub fn  blockchain_factory_file(filename:String,difficulty:String){
//     //web_sys::console::log_1(&"Called blockchain_factory_file!".into());
//     let mut difficulty_str=String::new();
//     let file_name = filename.trim().to_lowercase();
//     let blockchain = Blockchain::new();
//     //    
//     if difficulty.is_empty() {
//         difficulty_str = blockchain_executive::blockchain_var_ret_difficulty("0x00ffffffffffffffffffffffffffffff");       
//     }

//     let difficulty = string_to_u128(&difficulty_str);
//     //
//     blockchain_factory(blockchain, difficulty, || {
//         webasm_blockchain_sample_trx_json_data_block_from_file(&filename)
//     }).unwrap();
// }

// #[wasm_bindgen]
// pub fn  blockchain_factory_string(content:String,difficulty:String){
//     //web_sys::console::log_1(&"Called blockchain_factory_string!".into());
//     let mut difficulty_str=String::new();    
//     let blockchain = Blockchain::new();
//     //    
//     if difficulty.is_empty() {
//         difficulty_str = blockchain_executive::blockchain_var_ret_difficulty("0x00ffffffffffffffffffffffffffffff");      
//     }

//     let difficulty = string_to_u128(&difficulty_str);
//     //
//     blockchain_factory(blockchain, difficulty, || {
//         webasm_blockchain_sample_trx_json_data_from_string(&content)
//     }).unwrap();
// }
// //#[wasm_bindgen]
//  pub fn webasm_blockchain_sample_trx_json_data_block_from_file(filename: &str) -> Result<serde_json::Value, CustomError> {
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);
//     let serde_values_transactions = serde_json::from_reader(reader)?;

//     return Ok(serde_values_transactions);
// }


// //#[wasm_bindgen]
//  pub fn webasm_blockchain_sample_trx_json_data_from_string(content:&str) ->    Result<serde_json::Value, CustomError>   {
//     if !content.is_empty(){
//         let js = serde_json::from_str(content).unwrap();
//         return Ok(js);
//     }
//     let json = r#"{
//     "blocks":[{    
//         "block1":[{
//             "transactions":[{
//                     "transaction1":[{
//                         "inputs":[{                          
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "1000"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "1000"
//                         }]
//                     }]
//             }]
//         }]
//     }]
//     }
//     "#;

//     let js = serde_json::from_str(json).unwrap();

//     Ok(js)
// }



