extern crate wasm_bindgen;
extern crate serde;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Cov {
    path: String,
    #[serde(rename = "statementMap")]
    statement_map: Vec<StartEnd>,
    #[serde(rename = "fnMap")]
    fn_map:Vec<FnMap>,
    s: Vec<u16>,
    f: Vec<u16>,
    b: Vec<Vec<u16>>,
}

#[derive(Serialize, Deserialize)]
struct FnMap{
    name:String,
    decl: StartEnd,
    loc: StartEnd,
    line: u16
}

#[derive(Serialize, Deserialize)]
struct LineColumn{
    line:u16,
    column:u16
}

#[derive(Serialize, Deserialize)]
struct StartEnd {
    start: LineColumn,
    end: LineColumn
}

#[derive(Serialize, Deserialize)]
struct Params {
    a: u16,
    b: u16,
}

#[wasm_bindgen]
pub fn merge(params0: &str,params1: &str) -> String {
    internal_merge(params0,params1)
}

fn internal_merge(params0: &str,params1: &str) -> String {
    let mut first:Cov = serde_json::from_str(params0).unwrap();
    let second:Cov = serde_json::from_str(params1).unwrap();
    for i in 0..second.s.len() {
        first.s[i] = first.s[i] + second.s[i]
    }
    for i in 0..second.f.len() {
        first.f[i] = first.f[i] + second.f[i]
    }
    for i in 0..second.b.len() {
        for j in 0..second.b[i].len() {
            first.b[i][j] = first.b[i][j] + second.b[i][j]
        }
    }
    let sdata = serde_json::to_string(&first);
    if sdata.is_err() {
        println!("Error, failed to serialize structure: {}", sdata.unwrap_err());
        std::process::exit(1);
    }
    let sdata = sdata.unwrap();
    return sdata;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let data0 = r#"
        {"path":"client/framework/tools/type/index.js","statementMap":[{"start":{"line":6,"column":11},"end":{"line":8,"column":1}},{"start":{"line":7,"column":4},"end":{"line":7,"column":98}},{"start":{"line":10,"column":15},"end":{"line":14,"column":1}},{"start":{"line":11,"column":4},"end":{"line":13,"column":6}},{"start":{"line":12,"column":8},"end":{"line":12,"column":96}},{"start":{"line":12,"column":36},"end":{"line":12,"column":94}},{"start":{"line":16,"column":16},"end":{"line":19,"column":1}},{"start":{"line":17,"column":15},"end":{"line":17,"column":66}},{"start":{"line":18,"column":4},"end":{"line":18,"column":42}},{"start":{"line":21,"column":17},"end":{"line":21,"column":33}},{"start":{"line":23,"column":17},"end":{"line":23,"column":33}},{"start":{"line":25,"column":18},"end":{"line":25,"column":35}},{"start":{"line":27,"column":17},"end":{"line":27,"column":33}},{"start":{"line":29,"column":22},"end":{"line":29,"column":36}},{"start":{"line":31,"column":16},"end":{"line":31,"column":31}},{"start":{"line":33,"column":19},"end":{"line":33,"column":54}},{"start":{"line":34,"column":18},"end":{"line":40,"column":1}},{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},{"start":{"line":36,"column":8},"end":{"line":36,"column":57}},{"start":{"line":39,"column":4},"end":{"line":39,"column":17}},{"start":{"line":46,"column":16},"end":{"line":71,"column":1}},{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},{"start":{"line":50,"column":12},"end":{"line":50,"column":72}},{"start":{"line":51,"column":12},"end":{"line":51,"column":24}},{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},{"start":{"line":55,"column":12},"end":{"line":55,"column":48}},{"start":{"line":56,"column":12},"end":{"line":56,"column":24}},{"start":{"line":58,"column":8},"end":{"line":62,"column":10}},{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},{"start":{"line":64,"column":23},"end":{"line":64,"column":36}},{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},{"start":{"line":67,"column":8},"end":{"line":67,"column":57}},{"start":{"line":68,"column":8},"end":{"line":68,"column":20}},{"start":{"line":70,"column":4},"end":{"line":70,"column":30}},{"start":{"line":77,"column":21},"end":{"line":80,"column":1}},{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},{"start":{"line":78,"column":49},"end":{"line":78,"column":62}},{"start":{"line":79,"column":4},"end":{"line":79,"column":53}},{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},{"start":{"line":82,"column":27},"end":{"line":82,"column":40}},{"start":{"line":83,"column":14},"end":{"line":83,"column":49}},{"start":{"line":84,"column":4},"end":{"line":84,"column":32}},{"start":{"line":85,"column":4},"end":{"line":85,"column":24}}],"fnMap":[{"name":"(anonymous_0)","decl":{"start":{"line":6,"column":11},"end":{"line":6,"column":12}},"loc":{"start":{"line":6,"column":26},"end":{"line":8,"column":1}},"line":6},{"name":"(anonymous_1)","decl":{"start":{"line":10,"column":15},"end":{"line":10,"column":16}},"loc":{"start":{"line":10,"column":29},"end":{"line":14,"column":1}},"line":10},{"name":"(anonymous_2)","decl":{"start":{"line":11,"column":11},"end":{"line":11,"column":12}},"loc":{"start":{"line":11,"column":18},"end":{"line":13,"column":5}},"line":11},{"name":"(anonymous_3)","decl":{"start":{"line":12,"column":28},"end":{"line":12,"column":29}},"loc":{"start":{"line":12,"column":36},"end":{"line":12,"column":94}},"line":12},{"name":"(anonymous_4)","decl":{"start":{"line":16,"column":16},"end":{"line":16,"column":17}},"loc":{"start":{"line":16,"column":25},"end":{"line":19,"column":1}},"line":16},{"name":"(anonymous_5)","decl":{"start":{"line":34,"column":18},"end":{"line":34,"column":19}},"loc":{"start":{"line":34,"column":27},"end":{"line":40,"column":1}},"line":34},{"name":"(anonymous_6)","decl":{"start":{"line":46,"column":16},"end":{"line":46,"column":17}},"loc":{"start":{"line":46,"column":36},"end":{"line":71,"column":1}},"line":46},{"name":"(anonymous_7)","decl":{"start":{"line":77,"column":21},"end":{"line":77,"column":22}},"loc":{"start":{"line":77,"column":31},"end":{"line":80,"column":1}},"line":77},{"name":"isEmptyFunction","decl":{"start":{"line":81,"column":9},"end":{"line":81,"column":24}},"loc":{"start":{"line":81,"column":31},"end":{"line":86,"column":1}},"line":81}],"branchMap":[{"loc":{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},"type":"if","locations":[{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},{"start":{},"end":{}}],"line":35},{"loc":{"start":{"line":35,"column":8},"end":{"line":35,"column":51}},"type":"binary-expr","locations":[{"start":{"line":35,"column":8},"end":{"line":35,"column":22}},{"start":{"line":35,"column":26},"end":{"line":35,"column":51}}],"line":35},{"loc":{"start":{"line":36,"column":15},"end":{"line":36,"column":56}},"type":"binary-expr","locations":[{"start":{"line":36,"column":15},"end":{"line":36,"column":20}},{"start":{"line":36,"column":24},"end":{"line":36,"column":56}}],"line":36},{"loc":{"start":{"line":46,"column":23},"end":{"line":46,"column":31}},"type":"default-arg","locations":[{"start":{"line":46,"column":29},"end":{"line":46,"column":31}}],"line":46},{"loc":{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},"type":"if","locations":[{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},{"start":{},"end":{}}],"line":47},{"loc":{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},"type":"if","locations":[{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},{"start":{},"end":{}}],"line":48},{"loc":{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},"type":"if","locations":[{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},{"start":{},"end":{}}],"line":53},{"loc":{"start":{"line":53,"column":12},"end":{"line":53,"column":43}},"type":"binary-expr","locations":[{"start":{"line":53,"column":12},"end":{"line":53,"column":22}},{"start":{"line":53,"column":26},"end":{"line":53,"column":43}}],"line":53},{"loc":{"start":{"line":59,"column":12},"end":{"line":61,"column":30}},"type":"binary-expr","locations":[{"start":{"line":59,"column":12},"end":{"line":59,"column":59}},{"start":{"line":60,"column":12},"end":{"line":60,"column":35}},{"start":{"line":61,"column":12},"end":{"line":61,"column":30}}],"line":59},{"loc":{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},"type":"if","locations":[{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},{"start":{},"end":{}}],"line":64},{"loc":{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},"type":"if","locations":[{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},{"start":{},"end":{}}],"line":65},{"loc":{"start":{"line":65,"column":8},"end":{"line":65,"column":52}},"type":"binary-expr","locations":[{"start":{"line":65,"column":8},"end":{"line":65,"column":32}},{"start":{"line":65,"column":36},"end":{"line":65,"column":52}}],"line":65},{"loc":{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},"type":"if","locations":[{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},{"start":{},"end":{}}],"line":78},{"loc":{"start":{"line":78,"column":8},"end":{"line":78,"column":47}},"type":"binary-expr","locations":[{"start":{"line":78,"column":8},"end":{"line":78,"column":28}},{"start":{"line":78,"column":32},"end":{"line":78,"column":47}}],"line":78},{"loc":{"start":{"line":79,"column":14},"end":{"line":79,"column":51}},"type":"binary-expr","locations":[{"start":{"line":79,"column":14},"end":{"line":79,"column":22}},{"start":{"line":79,"column":26},"end":{"line":79,"column":51}}],"line":79},{"loc":{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},"type":"if","locations":[{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},{"start":{},"end":{}}],"line":82}],"s":[1,165,1,7,263,263,1,0,0,1,1,1,1,1,1,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,3,0,3,0,0,0,0,0],"f":[165,7,263,263,0,0,0,3,0],"b":[[0,0],[0,0],[0,0],[0],[0,0],[0,0],[0,0],[0,0],[0,0,0],[0,0],[0,0],[0,0],[0,3],[3,3],[3,3],[0,0]],"_coverageSchema":"1a1c01bbd47fc00a2c39e90264f33305004495a9","hash":"6a97254bb5b4167b8fd4cb5aa7722555e0c7266e"}"#;
        let data1 = r#"
        {"path":"client/framework/tools/type/index.js","statementMap":[{"start":{"line":6,"column":11},"end":{"line":8,"column":1}},{"start":{"line":7,"column":4},"end":{"line":7,"column":98}},{"start":{"line":10,"column":15},"end":{"line":14,"column":1}},{"start":{"line":11,"column":4},"end":{"line":13,"column":6}},{"start":{"line":12,"column":8},"end":{"line":12,"column":96}},{"start":{"line":12,"column":36},"end":{"line":12,"column":94}},{"start":{"line":16,"column":16},"end":{"line":19,"column":1}},{"start":{"line":17,"column":15},"end":{"line":17,"column":66}},{"start":{"line":18,"column":4},"end":{"line":18,"column":42}},{"start":{"line":21,"column":17},"end":{"line":21,"column":33}},{"start":{"line":23,"column":17},"end":{"line":23,"column":33}},{"start":{"line":25,"column":18},"end":{"line":25,"column":35}},{"start":{"line":27,"column":17},"end":{"line":27,"column":33}},{"start":{"line":29,"column":22},"end":{"line":29,"column":36}},{"start":{"line":31,"column":16},"end":{"line":31,"column":31}},{"start":{"line":33,"column":19},"end":{"line":33,"column":54}},{"start":{"line":34,"column":18},"end":{"line":40,"column":1}},{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},{"start":{"line":36,"column":8},"end":{"line":36,"column":57}},{"start":{"line":39,"column":4},"end":{"line":39,"column":17}},{"start":{"line":46,"column":16},"end":{"line":71,"column":1}},{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},{"start":{"line":50,"column":12},"end":{"line":50,"column":72}},{"start":{"line":51,"column":12},"end":{"line":51,"column":24}},{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},{"start":{"line":55,"column":12},"end":{"line":55,"column":48}},{"start":{"line":56,"column":12},"end":{"line":56,"column":24}},{"start":{"line":58,"column":8},"end":{"line":62,"column":10}},{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},{"start":{"line":64,"column":23},"end":{"line":64,"column":36}},{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},{"start":{"line":67,"column":8},"end":{"line":67,"column":57}},{"start":{"line":68,"column":8},"end":{"line":68,"column":20}},{"start":{"line":70,"column":4},"end":{"line":70,"column":30}},{"start":{"line":77,"column":21},"end":{"line":80,"column":1}},{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},{"start":{"line":78,"column":49},"end":{"line":78,"column":62}},{"start":{"line":79,"column":4},"end":{"line":79,"column":53}},{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},{"start":{"line":82,"column":27},"end":{"line":82,"column":40}},{"start":{"line":83,"column":14},"end":{"line":83,"column":49}},{"start":{"line":84,"column":4},"end":{"line":84,"column":32}},{"start":{"line":85,"column":4},"end":{"line":85,"column":24}}],"fnMap":[{"name":"(anonymous_0)","decl":{"start":{"line":6,"column":11},"end":{"line":6,"column":12}},"loc":{"start":{"line":6,"column":26},"end":{"line":8,"column":1}},"line":6},{"name":"(anonymous_1)","decl":{"start":{"line":10,"column":15},"end":{"line":10,"column":16}},"loc":{"start":{"line":10,"column":29},"end":{"line":14,"column":1}},"line":10},{"name":"(anonymous_2)","decl":{"start":{"line":11,"column":11},"end":{"line":11,"column":12}},"loc":{"start":{"line":11,"column":18},"end":{"line":13,"column":5}},"line":11},{"name":"(anonymous_3)","decl":{"start":{"line":12,"column":28},"end":{"line":12,"column":29}},"loc":{"start":{"line":12,"column":36},"end":{"line":12,"column":94}},"line":12},{"name":"(anonymous_4)","decl":{"start":{"line":16,"column":16},"end":{"line":16,"column":17}},"loc":{"start":{"line":16,"column":25},"end":{"line":19,"column":1}},"line":16},{"name":"(anonymous_5)","decl":{"start":{"line":34,"column":18},"end":{"line":34,"column":19}},"loc":{"start":{"line":34,"column":27},"end":{"line":40,"column":1}},"line":34},{"name":"(anonymous_6)","decl":{"start":{"line":46,"column":16},"end":{"line":46,"column":17}},"loc":{"start":{"line":46,"column":36},"end":{"line":71,"column":1}},"line":46},{"name":"(anonymous_7)","decl":{"start":{"line":77,"column":21},"end":{"line":77,"column":22}},"loc":{"start":{"line":77,"column":31},"end":{"line":80,"column":1}},"line":77},{"name":"isEmptyFunction","decl":{"start":{"line":81,"column":9},"end":{"line":81,"column":24}},"loc":{"start":{"line":81,"column":31},"end":{"line":86,"column":1}},"line":81}],"branchMap":[{"loc":{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},"type":"if","locations":[{"start":{"line":35,"column":4},"end":{"line":37,"column":5}},{"start":{},"end":{}}],"line":35},{"loc":{"start":{"line":35,"column":8},"end":{"line":35,"column":51}},"type":"binary-expr","locations":[{"start":{"line":35,"column":8},"end":{"line":35,"column":22}},{"start":{"line":35,"column":26},"end":{"line":35,"column":51}}],"line":35},{"loc":{"start":{"line":36,"column":15},"end":{"line":36,"column":56}},"type":"binary-expr","locations":[{"start":{"line":36,"column":15},"end":{"line":36,"column":20}},{"start":{"line":36,"column":24},"end":{"line":36,"column":56}}],"line":36},{"loc":{"start":{"line":46,"column":23},"end":{"line":46,"column":31}},"type":"default-arg","locations":[{"start":{"line":46,"column":29},"end":{"line":46,"column":31}}],"line":46},{"loc":{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},"type":"if","locations":[{"start":{"line":47,"column":4},"end":{"line":63,"column":5}},{"start":{},"end":{}}],"line":47},{"loc":{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},"type":"if","locations":[{"start":{"line":48,"column":8},"end":{"line":52,"column":9}},{"start":{},"end":{}}],"line":48},{"loc":{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},"type":"if","locations":[{"start":{"line":53,"column":8},"end":{"line":57,"column":9}},{"start":{},"end":{}}],"line":53},{"loc":{"start":{"line":53,"column":12},"end":{"line":53,"column":43}},"type":"binary-expr","locations":[{"start":{"line":53,"column":12},"end":{"line":53,"column":22}},{"start":{"line":53,"column":26},"end":{"line":53,"column":43}}],"line":53},{"loc":{"start":{"line":59,"column":12},"end":{"line":61,"column":30}},"type":"binary-expr","locations":[{"start":{"line":59,"column":12},"end":{"line":59,"column":59}},{"start":{"line":60,"column":12},"end":{"line":60,"column":35}},{"start":{"line":61,"column":12},"end":{"line":61,"column":30}}],"line":59},{"loc":{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},"type":"if","locations":[{"start":{"line":64,"column":4},"end":{"line":64,"column":36}},{"start":{},"end":{}}],"line":64},{"loc":{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},"type":"if","locations":[{"start":{"line":65,"column":4},"end":{"line":69,"column":5}},{"start":{},"end":{}}],"line":65},{"loc":{"start":{"line":65,"column":8},"end":{"line":65,"column":52}},"type":"binary-expr","locations":[{"start":{"line":65,"column":8},"end":{"line":65,"column":32}},{"start":{"line":65,"column":36},"end":{"line":65,"column":52}}],"line":65},{"loc":{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},"type":"if","locations":[{"start":{"line":78,"column":4},"end":{"line":78,"column":62}},{"start":{},"end":{}}],"line":78},{"loc":{"start":{"line":78,"column":8},"end":{"line":78,"column":47}},"type":"binary-expr","locations":[{"start":{"line":78,"column":8},"end":{"line":78,"column":28}},{"start":{"line":78,"column":32},"end":{"line":78,"column":47}}],"line":78},{"loc":{"start":{"line":79,"column":14},"end":{"line":79,"column":51}},"type":"binary-expr","locations":[{"start":{"line":79,"column":14},"end":{"line":79,"column":22}},{"start":{"line":79,"column":26},"end":{"line":79,"column":51}}],"line":79},{"loc":{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},"type":"if","locations":[{"start":{"line":82,"column":4},"end":{"line":82,"column":40}},{"start":{},"end":{}}],"line":82}],"s":[1,165,1,7,263,263,1,0,0,1,1,1,1,1,1,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,3,0,3,0,0,0,0,0],"f":[165,7,263,263,0,0,0,3,0],"b":[[0,0],[0,0],[0,0],[0],[0,0],[0,0],[0,0],[0,0],[0,0,0],[0,0],[0,0],[0,0],[0,3],[3,3],[3,3],[0,0]],"_coverageSchema":"1a1c01bbd47fc00a2c39e90264f33305004495a9","hash":"6a97254bb5b4167b8fd4cb5aa7722555e0c7266e"}"#;
        assert_eq!("3", internal_merge(data0,data1));
    }
}
