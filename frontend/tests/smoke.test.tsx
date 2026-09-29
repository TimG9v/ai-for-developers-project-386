import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import Page from "@/app/page";

describe("home page", () => {
  it("shows the 'Create event' button", () => {
    render(<Page />);

    // getByRole throws if the button is absent or hidden from the a11y tree
    expect(screen.getByRole("button", { name: "Create event" })).toBeTruthy();
  });
});
