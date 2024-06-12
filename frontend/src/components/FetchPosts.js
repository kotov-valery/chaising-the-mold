import React, { useState, useEffect } from "react";
import axios from "axios";

function FetchPosts() {
  const [post, setPost] = useState({});
  const [id, setId] = useState(1);
  const [idFromButtonClick, setIdFromButtonClick] = useState(1);

  useEffect(() => {
    axios
      .get(`https://jsonplaceholder.typicode.com/posts/${idFromButtonClick}`)
      .then((response) => {
        setPost(response.data);
      })
      .catch((error) => {
        console.error("Error fetching data: ", error);
      });
  }, [idFromButtonClick]);

  return (
    <div>
      <h1>Posts</h1>
      <input type="text" value={id} onChange={(e) => setId(e.target.value)} />
      <button onClick={() => setIdFromButtonClick(id)}>Fetch Post</button>
      <p>
        <strong>Title:</strong> {post.title}
        <hr />
        {post.body}
      </p>
    </div>
  );
}

export default FetchPosts;
