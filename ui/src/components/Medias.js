import React, { useEffect, useState, useRef } from "react";

import Media from "./Media";

const Medias = ({ search }) => {
  const [skip, setSkip] = useState(0);
  const [term, setTerm] = useState("");
  const [cards, setCards] = useState([]);

  // Setup our loader ref.
  const loader = useRef(null);
  useEffect(() => {
    const options = {
      root: null,
      rootMargin: "20px",
      threshold: 1.0,
    };
    const observer = new IntersectionObserver(handleObserver, options);
    if (loader.current) observer.observe(loader.current);
  }, []);
  const handleObserver = (entities) => {
    const target = entities[0];
    if (target.isIntersecting) setSkip((skip) => skip + 20);
  };

  // When our search changes, we should reset.
  useEffect(() => {setTerm(search);setSkip(0);setCards([]);}, [search]);
  
  useEffect(() => {
	let url = new URL(
      "/api/media",
      window.location.protocol + "//" + window.location.host,
    );
    let params = { search: term, skip: skip, limit: 20 };
    url.search = new URLSearchParams(params).toString();
    fetch(url)
      .then((response) => response.json())
      .then((data) => {
        setCards((cards) => [
          ...cards,
          ...data.map((image) => <Media image={image} key={image._id} />),
        ]);
      });
  }, [term, skip]);

  return (
    <div className="container-fluid">
      <div className="column">
        <div className="row">{cards}</div>
        <div className="row" ref={loader}>
        </div>
      </div>
    </div>
  );
};

export default Medias;
