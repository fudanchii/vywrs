import "./assets/main.scss";
import("./pkg").then(module => {
  module.entrypoint();
});
