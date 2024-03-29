var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
let UploadForm = class UploadForm extends LitElement {
    encryptionType = '';
    formData = new FormData();
    responseData = null;
    static get styles() {
        return css `
      .container {
        max-width: 600px;
        margin: 0 auto;
        padding: 20px;
      }
      .my-4 {
        margin-top: 2rem;
        margin-bottom: 2rem;
      }
      .form-check {
        margin-left: 20px;
      }
      .btn {
        margin-top: 20px;
      }
      .mt-4 {
        margin-top: 2rem;
      }
    `;
    }
    handleFileChange(event) {
        const file = event.target.files?.[0];
        if (file) {
            this.formData.set('file', file);
        }
    }
    handleSubmit(event) {
        event.preventDefault();
        const url = `http://localhost:8080${this.encryptionType ? `/?encryption=${this.encryptionType}` : ''}`;
        fetch(url, {
            method: 'POST',
            body: this.formData,
        })
            .then((response) => response.json())
            .then((data) => {
            console.log(data);
            this.responseData = data;
            this.requestUpdate();
        })
            .catch((error) => {
            console.error(error);
        });
    }
    update(changedProperties) {
        super.update(changedProperties);
        console.log('updated', changedProperties);
    }
    render() {
        return html `
      <div class="container">
        <h1 class="my-4">File Upload Form</h1>
        <form @submit=${this.handleSubmit}>
          <div class="mb-3">
            <label for="file" class="form-label">Choose a file:</label>
            <input
              type="file"
              class="form-control"
              id="file"
              name="file"
              @change=${this.handleFileChange}
            />
          </div>
          <div class="mb-3">
            <label for="encryption" class="form-label">Encryption:</label>
            <div class="form-check">
              <input
                class="form-check-input"
                type="radio"
                name="encryption"
                id="none"
                value=""
                .checked=${!this.encryptionType}
                @change=${() => (this.encryptionType = '')}
              />
              <label class="form-check-label" for="none">
                None
              </label>
            </div>
            <div class="form-check">
              <input
                class="form-check-input"
                type="radio"
                name="encryption"
                id="aes"
                value="aes"
                .checked=${this.encryptionType === 'aes'}
                @change=${() => (this.encryptionType = 'aes')}
              />
              <label class="form-check-label" for="aes">
                AES
              </label>
            </div>
          </div>
          <button type="submit" class="btn btn-primary">Upload</button>
        </form>
        ${this.responseData
            ? html `
              <div class="mt-4">
                <h2 class="my-4">Response Data:</h2>
                <pre>${JSON.stringify(this.responseData, null, 2)}</pre>
              </div>
            `
            : ''}
      </div>
    `;
    }
};
__decorate([
    property({ type: String })
], UploadForm.prototype, "encryptionType", void 0);
__decorate([
    state()
], UploadForm.prototype, "formData", void 0);
__decorate([
    state()
], UploadForm.prototype, "responseData", void 0);
UploadForm = __decorate([
    customElement('upload-form')
], UploadForm);
export { UploadForm };
//# sourceMappingURL=upload-form.js.map