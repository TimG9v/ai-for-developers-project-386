import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import Page from "@/app/booking/page";

afterEach(cleanup);

describe("booking page", () => {
  it("renders the page heading", () => {
    render(<Page />);

    expect(
      screen.getByRole("heading", { level: 1, name: "Страница записи" }),
    ).toBeTruthy();
  });
});
