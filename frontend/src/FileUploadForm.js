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
        setResponseData(data);
      })
      .catch(error => {
        console.error(error);
      });
  }

  return (
    <div className="container">
      <h1 className="my-4">File Upload Form</h1>
      <form onSubmit={handleSubmit}>
        <div className="mb-3">
          <label htmlFor="file" className="form-label">Choose a file:</label>
          <input type="file" className="form-control" id="file" name="file" onChange={handleFileChange} />
        </div>
        <div className="mb-3">
          <label htmlFor="encryption" className="form-label">Encryption:</label>
          <div className="form-check">
            <input className="form-check-input" type="radio" name="encryption" id="none" value="" checked={!encryptionType} onChange={() => setEncryptionType("")} />
            <label className="form-check-label" htmlFor="none">
              None
            </label>
          </div>
          <div className="form-check">
            <input className="form-check-input" type="radio" name="encryption" id="aes" value="aes" checked={encryptionType === "aes"} onChange={() => setEncryptionType("aes")} />
            <label className="form-check-label" htmlFor="aes">
              AES
            </label>
          </div>
        </div>
        <button type="submit" className="btn btn-primary">Upload</button>
      </form>
      {responseData && (
        <div className="mt-4">
          <h2 className="my-4">Response Data:</h2>
          <pre>{JSON.stringify(responseData, null, 2)}</pre>
        </div>
      )}
    </div>
  );
}

export default UploadForm;