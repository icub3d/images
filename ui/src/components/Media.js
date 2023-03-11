import React from "react";

const Media = ({ image }) => {
  let m = (
    <img
      src={`https://img.marsh.gg/${image._id}`}
      alt={image._id}
      className="img-thumbnail"
    />
  );
  if (image.type.startsWith("video")) {
    m = (
      <video controls preload="metadata" className="img-thumbnail">
        <source src={`https://img.marsh.gg/${image._id}`} type={image.type} />
      </video>
    );
  } else if (!image.type.startsWith("image")) {
    m = (
      <i
        className="bi bi-file-earmark-binary"
        style={{ fontSize: "20rem" }}
      ></i>
    );
  }

  let bgs = ["primary", "secondary", "info", "warning", "danger"];

  return (
    <div
      className={`card bg-${bgs[Math.floor(Math.random() * bgs.length)]
        } text-dark m-1 p-2`}
      style={{ maxWidth: "20rem" }}
      onClick={() => window.open(`https://img.marsh.gg/${image._id}`, "_blank")}
    >
      {m}
      <div className="card-body">
        <h5 className="card-title">{image._id}</h5>
        <p className="card-text">{image.tags.join(", ")}</p>
      </div>
    </div>
  );
};

export default Media;
