import { Diagram } from "../features/diagram";
import { withProviders } from "./providers";

const App = () => {
  return <Diagram />;
};

export default withProviders(App);
