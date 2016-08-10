//! Bla bla
//! Bla bla bla

extern crate rustc_serialize;

mod workflow;

#[cfg(feature = "disp-name")]
mod display_name;
#[cfg(feature = "disp-uri")]
mod display_uri;

pub use self::workflow::*;


/// Represents a tree, see [`Tree::node`] and [`Tree::leaf`] functions
/// [`Tree::node`]: ./enum.Tree.html#method.node
/// [`Tree::leaf`]: ./enum.Tree.html#method.leaf
#[derive(Clone, Debug, RustcEncodable)]
pub enum Tree {
    Leaf(String),
    Node(String, Vec<Tree>),
}

impl Tree {
    /// Creates a [`Tree::Leaf`]
    /// [`Tree::Leaf`]: ./enum.Tree.html#variant.Leaf
    pub fn leaf(name: &str) -> Tree {
        Tree::Leaf(name.to_owned())
    }

    /// Creates a [`Tree::Node`]
    /// [`Tree::Node`]: ./enum.Tree.html#variant.Node
    pub fn node(name: &str, children: &[Tree]) -> Tree {
        Tree::Node(name.to_owned(), children.to_owned())
    }

    #[inline(never)]
    pub fn name(&self) -> &String {
        match *self {
            Tree::Leaf(ref name)    => name,
            Tree::Node(ref name, _) => name,
        }
    }
}

pub fn print_tree(tree: &Tree) {
    println!("{}", tree.name());
    print_tree_(tree, &mut Vec::new());
}

fn print_tree_(tree: &Tree, stack: &mut Vec<bool>) {
    match *tree {
        Tree::Leaf(_)               => {},
        Tree::Node(_, ref children) => {
            let len = children.len();
            for i in 0..len {
                let child = &children[i];
                println!("{} `-  {}", prefix(stack), child.name());

                stack.push(i == len - 1);
                print_tree_(child, stack);
                stack.pop();
            }
        }
    }
}

/// Generate a prefix String.
///
/// # Examples
///
/// ```
/// use first::prefix;
///
/// let vec = vec!(false, false);
/// assert_eq!(" |   |  ", prefix(&vec));
/// ```
///
/// ```
/// use first::prefix;
///
/// let vec = vec!(true, false);
/// assert_eq!("     |  ", prefix(&vec));
/// ```
pub fn prefix(stack: &Vec<bool>) -> String {
    stack.iter()
        .map(|x| if *x { "    " } else { " |  " })
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Debug)]
pub enum Foo {
    Bar,
    Baz,
}

impl std::str::FromStr for Foo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bar" | "Bar" => Ok(Foo::Bar),
            "baz" | "Baz" => Ok(Foo::Baz),
            _     => Err("parse error")
        }
    }
}

pub fn workflow_url(component_id: &str, endpoint_id: &str) -> String {
    format!("http://styx.spotify.net/api/v0/workflows/{}/{}", component_id, endpoint_id)
}
