import { useState } from "react";
import { Button, Form } from "react-bootstrap";
import Colorful from "@uiw/react-color-colorful";
import { postCreateTeam } from "lib/api";

export function CreateTeam() {
  const [color, setColor] = useState("#9900EF");
  const [name, setName] = useState("");
  const [loading, setLoading] = useState(false);

  const sendRequest = async () => {
    setLoading(true);
    await postCreateTeam(name, color)
      .then(() => {
        setLoading(false);
        window.location.href = "/";
      })
      .catch((err) => {
        setLoading(false);
        alert(err.response.data);
      });
  };

  return (
    <div
      className="d-flex flex-column justify-content-center align-items-center h-max"
      style={{ backgroundColor: color }}
    >
      <form
        className="bg-white p-3 rounded shadow-lg"
        onSubmit={(e) => {
          e.preventDefault();
          !loading && sendRequest();
        }}
      >
        <Form.Control
          placeholder="Team name"
          onChange={(item) => setName(item.target.value)}
        />
        <div className="d-flex flex-column justify-content-center align-items-center p-2">
          <Colorful
            className="m-2"
            disableAlpha
            onChange={(data) => setColor(data.hex)}
            color={color}
          />
        </div>
        <div className="d-grid gap-2">
          <Button disabled={loading} variant="primary" onClick={sendRequest}>
            {!loading ? (
              <>Create</>
            ) : (
              <div className="spinner-border spinner-border-sm" role="status">
                <span className="visually-hidden">Loading...</span>
              </div>
            )}
          </Button>
        </div>
      </form>
    </div>
  );
}
