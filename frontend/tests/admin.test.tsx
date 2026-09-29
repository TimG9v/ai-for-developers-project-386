import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import Page from "@/app/admin/page";

afterEach(cleanup);

describe("admin page", () => {
  it("renders the page heading", () => {
    render(<Page />);

    expect(
      screen.getByRole("heading", { level: 1, name: "Админка" }),
    ).toBeTruthy();
  });
});
