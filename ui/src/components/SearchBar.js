import React, { useState } from "react";

const SearchBar = ({ newSearch }) => {
  const [search, setSearch] = useState("");

  const onFormSubmit = (event) => {
    event.preventDefault();
    newSearch(search);
  };

  return (
    <form onSubmit={onFormSubmit}>
      <div className="form-group">
        <input
          className="form-control"
          placeholder="Search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
    </form>
  );
};

export default SearchBar;
