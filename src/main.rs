#![allow(dead_code, unused_variables)]
//! Bla bla bla.

#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate first;

use first::*;


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
            (@arg workflow: * "Workflow id")
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
        let json = serde_json::to_string(&tree).unwrap();
        println!("{}", json);

    } else if let Some(sub_tree) = matches.subcommand_matches("tree") {
        let pos = value_t!(sub_tree, "foo", Foo).unwrap_or(Foo::Baz);
        println!("{:?}", pos);
        print_tree(&tree);

    } else if let Some(sub_workflow) = matches.subcommand_matches("workflow") {
        let component_id = sub_workflow.value_of("component").unwrap();
        let workflow_id = sub_workflow.value_of("workflow").unwrap();

        let url = workflow_url(component_id, workflow_id);
        let mut response = reqwest::get(&url).unwrap();

        if !response.status().is_success() {
            println!("Service returned {}", response.status());
            std::process::exit(2);
        }

        let json: String = response.text().unwrap();
        match serde_json::from_str::<Workflow>(&json) {
            Ok(workflow) => println!("{:#?}", workflow),
            Err(err)     => println!("{:?}", err),
        }
    }
}
