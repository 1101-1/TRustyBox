## File Hosting with MongoDB Authentication, Axum and AES-256 encryption

This project is a simple file hosting service with encrypt files and built with Rust and MongoDB. It allows users to securely store and retrieve files using a RESTful API. The service authenticates users against a MongoDB database, ensuring that users can access their files.

### Setting up MongoDB

Before running the project, make sure you have MongoDB installed and running on your machine. You will also need to create a user and a database with authentication enabled. To do this, follow these steps:

1. Start the MongoDB shell by running `mongod` in your terminal.
2. Create a new database by running `use <database_name>` in the shell.
3. Create a new user with authentication privileges by running the following command:

   ```python
   db.createUser({
      user: "<username>",
      pwd: "<password>",
      roles: [{ role: "readWrite", db: "<database_name>" }]
    })
   ```

Make sure to replace `<username>`, `<password>`, and `<database_name>` with your desired values.

### Setting up `env.var("<name>")`
After setting up your MongoDB server, you need to change all strings like `env::var("NAME")` with a field with your data.

If you doing all correctly, your code will work.

`NOTE`: Field with `AES_KEY` need to insert the string with generated AES in the format like 
```rust
    let mut gen_aes_key = [0u8; 32];
    thread_rng().fill_bytes(&mut gen_aes_key);
    let aes_key_str = hex::encode(gen_aes_key);
    env::set_var("AES_KEY", &aes_key_str);
```
### Running the Project

To run the project, first clone the repository and navigate to the project directory. Then, run the following command in `cmd` to start the server:

`cargo run`

This will start the server on `http://127.0.0.1:8080/` or by DATA in `env::var("SERVER_ADDR")`.

### Uploading Files

To upload a file to the server, you can use `curl` in your terminal. For example, to upload a file called `example.txt`, run the following command:
`curl -X POST -F "file=@example.txt" http://127.0.0.1:8080/`


This will upload the file to the server and return a `URL` and other `DATA` in `JSON` format that you can use to GET the file by following the link:

In our case we need `<short_path>` and `<aes_key>`. We need to insert this data like:

`http://127.0.0.1:8080/<short_path>/<aes_key>/` where `<short_path>` is unique generated_path inseted in MongoDB and `<aes_key>` is generated aes_key, which sent to the user in `JSON` format

### Conclusion

With this project, you can easily create a secure file hosting service with `MongoDB` authentication and encrypted files. By following the steps outlined above, you can set up `MongoDB`, run the project, and start uploading files to your own secure file hosting service written on Axum.