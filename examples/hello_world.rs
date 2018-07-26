#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct HelloWorld;

/*
#[derive(Clone, Debug)]
pub struct GoodbyeWorld;

#[derive(Debug, Response)]
pub struct HelloResponse {
    msg: &'static str,
}

#[derive(Debug, Response)]
#[web(status = "201", header(name = "x-foo", value = "bar"))]
pub struct CreateResponse {
    msg: &'static str,

    #[web(header)]
    foo_bar: &'static str,
}

/*
#[derive(Debug, Response)]
pub struct User {
    id: usize,
}
*/
*/

#[derive(Debug, Extract)]
struct User {
    id: usize,
}

#[derive(Debug, Extract)]
struct MyArg {
    foo: String,
}

impl_web! {
    impl HelloWorld {
        /// @get("/hello-qs")
        /// @content_type("plain")
        fn hello_qs(&self, query_string: MyArg) -> Result<String, ()> {
            println!("QUERY: {:?}", query_string);
            Ok("We have received the query".to_string())
        }

        /// @post("/users")
        /// @content_type("plain")
        fn create_user(&self, body: User) -> Result<String, ()> {
            println!("GOT = {:?}", body);
            Ok("We have received the user".to_string())
        }

        /*
        /// @post("/input")
        fn post_input<S: BufStream>(&self, body: S, arg: MyArg) -> Result<String, ()> {
            unimplemented!();
        }

        /// @get("/")
        /// @content_type("json")
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }

        /// @get("/users/:id")
        /// @content_type("plain")
        fn get(&self, id: u32) -> Result<String, ()> {
            println!("GOT: id={:?};", id);
            Ok("ZOOOOMG YO\n".to_string())
        }

        /// @post("/users")
        /// @content_type("json")
        fn post(&self) -> Result<CreateResponse, ()> {
            Ok(CreateResponse {
                msg: "done",
                foo_bar: "winning",
            })
        }
        /*
        /// @get("/whoami")
        /// @content_type("plain")
        fn who_am_i(&mut self, user_agent: String) -> Result<String, ()> {
            Ok(format!("You are: {}\n", user_agent))
        }

        /// @get("/custom-header")
        /// @content_type("json")
        fn custom_header(&mut self, x_foo: &str) -> Result<String, ()> {
            Ok(format!("Sent: {}", x_foo))
        }
        */

        /*
        /// @get("/foo")
        /// @content_type("plain")
        fn qs(&mut self) -> Result<String, ()> {
        }
        */

        /*
        /// @GET("/authx")
        fn get(&mut self, content_length: HeaderValue) -> Result<String, ()> {
            unimplemented!();
        }
         */
        */
    }
}

/*
    impl GoodbyeWorld {
        /// @get("/goodbye")
        /// @content_type("plain")
        fn goodbye(&self) -> Result<String, ()> {
            Ok("Other resource".to_string())
            /*
            Ok(HelloResponse {
                msg: "other resource",
            })
            */
        }
    }
}
*/

pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    ServiceBuilder::new()
        .resource(HelloWorld)
        // .resource(GoodbyeWorld)
        .run(&addr)
        .unwrap();
}
