import { useState } from "react";
import "./App.css";

function App() {
  const [prompt, setPrompt] = useState("");
  const [result, setResult] = useState("");
  const [loading, setLoading] = useState(false);

  const handleParaphrase = async () => {
    if (!prompt.trim()) return;

    setLoading(true);
    try {
      const res = await fetch("http://localhost:3000/api/chat", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({ prompt }),
});

      const text = await res.text();
      setResult(text);
    } catch (err) {
      setResult("⚠️ Failed to connect to backend.");
      console.error(err);
    }
    setLoading(false);
  };

  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif" }}>
      <h2>📝 Paraphrase App</h2>
      <textarea
        value={prompt}
        onChange={(e) => setPrompt(e.target.value)}
        placeholder="Type or paste text to paraphrase..."
        rows={6}
        style={{ width: "100%", padding: "1rem", fontSize: "1rem" }}
      />
      <button
        onClick={handleParaphrase}
        style={{ marginTop: "1rem", padding: "0.5rem 1rem" }}
      >
        {loading ? "Paraphrasing..." : "Paraphrase"}
      </button>

      {result && (
        <div
          style={{
            marginTop: "2rem",
            padding: "1rem",
            background: "#eee",
            borderRadius: "8px",
          }}
        >
          <h3>🔁 Paraphrased Text:</h3>
          <p>{result}</p>
        </div>
      )}
    </div>
  );
}

export default App;
