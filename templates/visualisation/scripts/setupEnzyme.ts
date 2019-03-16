/* Used for tests by Jest to setup testing environment for react components */

import { configure } from "enzyme";
import * as EnzymeAdapter from "enzyme-adapter-react-16";
configure({ adapter: new EnzymeAdapter() });
