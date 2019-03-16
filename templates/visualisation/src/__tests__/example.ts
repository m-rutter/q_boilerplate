import * as React from "react";
import { shallow } from "enzyme";

describe("something", () => {
    test("blah", () => {
        expect(0).toBe(0);

        const div = shallow(<div>test</div>);

        expect(div.hasClass("test")).toBe(false);
    });
});
