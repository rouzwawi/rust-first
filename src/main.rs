#![allow(dead_code, unused_variables)]
//! Bla bla bla.

#[macro_use]
extern crate clap;
extern crate hyper;
extern crate rustc_serialize;
extern crate first;

use hyper::client::Client;
use hyper::client::Response;

use rustc_serialize::json;

use first::*;
use first::workflow::Workflow;


fn is_conf(val: String) -> Result<(), String> {
    if val.ends_with(".conf") {
        Ok(())
    } else {
        Err(String::from("Config must end with '.conf'"))
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Does things")

        // ... -> multiple occurrences
        (@arg verbose: -v ... "Sets the level of verbosity")
        (@arg flagga: -f ... "Sets som redundant flag")

        // #{x,y} -> bounds for value count
        // [file] -> explicit name, [] -> optional, <> -> required
        // {fn}   -> validation function : fn (String) -> Result<(), String>
        (@arg config: -c --config [file] #{1,2} {is_conf} "Set a custom config file")

        (@subcommand json =>
            (about: "Print as JSON")
        )
        (@subcommand tree =>
            (about: "Print as human readable tree")

            // * -> required
            (@arg foo: * "positional of type Foo")
        )
        (@subcommand workflow =>
            (about: "Show a workflow definition")

            // * -> required
            (@arg component: * "Component id")
            (@arg endpoint: * "Endpoint id")
        )
    ).get_matches();

    if matches.occurrences_of("verbose") > 0 {
        println!("{:?}", matches);
    }

    let tree = Tree::node("root", &[
        Tree::node("A", &[
            Tree::leaf("AA"),
            Tree::leaf("AB"),
        ]),
        Tree::node("B", &[
            Tree::leaf("BA"),
            Tree::leaf("BB"),
        ]),
    ]);

    if let Some(sub_json) = matches.subcommand_matches("json") {
        let json = json::encode(&tree).unwrap();
        println!("{}", json);

    } else if let Some(sub_tree) = matches.subcommand_matches("tree") {
        let pos = value_t!(sub_tree, "foo", Foo).unwrap_or(Foo::Baz);
        println!("{:?}", pos);
        print_tree(&tree);

    } else if let Some(sub_workflow) = matches.subcommand_matches("workflow") {
        use std::io::Read;

        let component_id = sub_workflow.value_of("component").unwrap();
        let endpoint_id = sub_workflow.value_of("endpoint").unwrap();

        let client = Client::new();
        let mut response: Response = client
            .get(&workflow_url(component_id, endpoint_id))
            .send().unwrap();

        if response.status != hyper::Ok {
            println!("Service returned {}", response.status);
            std::process::exit(2);
        }

        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        match json::decode::<Workflow>(&buffer) {
            Ok(workflow) => println!("{:#?}", workflow),
            Err(err)     => println!("{:?}", err),
        }

    }
}
