![TRustyBox](https://github.com/1101-1/TRustyBox/blob/main/logo.png)
# File Hosting with MongoDB Authentication, Axum and AES-256 encryption

This project is a simple file hosting service with encrypt files and built with Rust and MongoDB. It allows users to securely store and retrieve files using a RESTful API. The service authenticates users against a MongoDB database, ensuring that users can access their files. 

It can be easily up by docker compose.


## Easy Up
### Docker Compose Configuration

To easily host the entire application with three containers (MongoDB, frontend, and backend), you can use the provided docker-compose.yml file. It will ensure all the services are running and connected correctly.

Make sure you have Docker and Docker Compose installed, then run the following command:

```bash
docker-compose up
```
This will build and start all the containers, and your file hosting service will be available according to the specified ports and addresses in the docker-compose.yml file.

Remember to replace any placeholders in the docker-compose.yml file with your actual values as needed.

## Setting up MongoDB

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

## Setting up `env::var("<name>")`
After setting up your MongoDB server, you need to change all strings like `env::var("NAME")` with a field with your data.

If you doing all correctly, your code will work.

`NOTE`: Field with `AES_KEY` need to insert the string with generated AES in the format like 
```rust
 let mut gen_aes_key = [0u8; 32];
 thread_rng().fill_bytes(&mut gen_aes_key);
 let aes_key_str = general_purpose::STANDARD_NO_PAD.encode(gen_aes_key);
 env::set_var("AES_KEY", &aes_key_str);
```
## Running the Project
### `BACKEND`
To run the project, first clone the repository and navigate to the project directory. Then, run the following command in `cmd` to start the server:

`cargo run`

This will start the server on `http://127.0.0.1:8080/` or by DATA in `env::var("SERVER_ADDR")`.

### `FRONTEND`
Second step is go to the `frontend` directory by `cd frontend/` command on Linux system.

First, you need to change `upload-form.ts` file.

```Typescript
const url = `<addr>${
    this.encryptionType ? `/?encryption=${this.encryptionType}` : ''
  }`;
```
where <addr> is your server addres like `127.0.0.1:8080`.

In `frontend` directory, you need to write `npm run build` and after `npm run dev` and frontend part will be started.

## Uploading Files

### `BACKEND PART`

To upload a file to the server, you can use `curl` in your terminal as one of the ways. For example, to upload a file called `example.txt` without encryption by default, you can run the following command:

`curl -X POST -F "file=@example.txt" http://127.0.0.1:8080/`


Otherwise, if we need to upload encrypted file on server, we can use query. Here an instance:

`curl -X POST -F "file=@example.txt" http://127.0.0.1:8080/?encryption=aes/`

### `FRONTEND PART`
Second way, is go to address where your `frontend` part starts.

By standart is `127.0.0.1:5173`.

## Download Files

### `BACKEND PART`
To get file from server if you used cURL, we need to go on the following link:

`http://127.0.0.1:8080/<short_path>/`

If file was encrypted, instead need to use `http://127.0.0.1:8080/<short_path>/<aes_key>`.

In our case  `<short_path>` is unique generated_path inserted in MongoDB and `<aes_key>` is generated <aes_key> by server, which is not stored it.

### `FRONTEND PART`

If we use frontend part, you can see needed information in `Response Data` field.

## Additional info's
You also can check my [telegram bot](https://github.com/1101-1/TRustyBox_telegram_bot), that works with TRustyBox.