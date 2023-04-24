import React, { useState } from "react";

function UploadForm() {
  const [encryptionType, setEncryptionType] = useState("");
  const [formData] = useState(new FormData());
  const [responseData, setResponseData] = useState(null);

  function handleFileChange(event) {
    const file = event.target.files[0];
    formData.set("file", file);
  }

  function handleSubmit(event) {
    event.preventDefault();
    const url = `http://localhost:8080${encryptionType ? `/?encryption=${encryptionType}` : ""}`;
    fetch(url, {
      method: "POST",
      body: formData
    })
      .then(response => response.json())
      .then(data => {
        console.log(data);
        setResponseData(data); // Set the response data in state
      })
      .catch(error => {
        console.error(error);
      });
  }

  return (
    <div>
      <h1>File Upload Form</h1>
      <form action="/" method="post" encType="multipart/form-data" onSubmit={handleSubmit}>
        <input type="file" name="file" onChange={handleFileChange} />
        <input type="submit" value="Upload" />
      </form>
      <div>
        <input
          type="radio"
          id="aes"
          name="encryption"
          value="aes"
          checked={encryptionType === "aes"}
          onChange={() => setEncryptionType("aes")}
        />
        <label htmlFor="aes">AES</label>
        <input
          type="radio"
          id="none"
          name="encryption"
          value=""
          checked={!encryptionType}
          onChange={() => setEncryptionType("")}
        />
        <label htmlFor="none">None</label>
      </div>
      {responseData && <pre>{JSON.stringify(responseData, null, 2)}</pre>} {/* Render the response data */}
    </div>
  );
}

export default UploadForm;