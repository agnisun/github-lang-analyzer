export const fetchByUrl = (url) => {
  return fetch(url).then((res) => res.json());
};
