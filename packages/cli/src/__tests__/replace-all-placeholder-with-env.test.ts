import { placeholder } from "../../../shared";
import { replaceAllPlaceholderWithEnv } from "../replace-all-placeholder-with-env";

afterEach(() => {
  jest.clearAllMocks();
});

describe("replaceAllPlaceholderWithEnv", () => {
  test("it replace placeholder with env", () => {
    // arrange
    const code = `
      const hello = '${placeholder}'.HELLO;
    `;
    const env = {
      HELLO: "world",
    };

    // act
    const result = replaceAllPlaceholderWithEnv(code, env);

    // assert
    expect(result).toMatchInlineSnapshot(`
      "
            const hello = '{\\"HELLO\\":\\"world\\"}'.HELLO;
          "
    `);
  });

  test("it replace all placeholder with env", () => {
    // arrange
    const code = `
      const foo = '${placeholder}'.FOO;
      const bar = '${placeholder}'.BAR;
    `;
    const env = {
      FOO: "foo",
      BAR: "bar",
    };

    // act
    const result = replaceAllPlaceholderWithEnv(code, env);

    // assert
    expect(result).toMatchInlineSnapshot(`
      "
            const foo = '{\\"FOO\\":\\"foo\\",\\"BAR\\":\\"bar\\"}'.FOO;
            const bar = '{\\"FOO\\":\\"foo\\",\\"BAR\\":\\"bar\\"}'.BAR;
          "
    `);
  });

  test("it works with single quotes", () => {
    // arrange
    const code = `
      const hello = '${placeholder}'.HELLO;
    `;
    const env = {
      HELLO: "world",
    };

    // act
    const result = replaceAllPlaceholderWithEnv(code, env);

    // assert
    expect(result).toMatchInlineSnapshot(`
      "
            const hello = '{\\"HELLO\\":\\"world\\"}'.HELLO;
          "
    `);
  });

  test("it works with double quotes", () => {
    // arrange
    const code = `
      const hello = "${placeholder}".HELLO;
    `;
    const env = {
      HELLO: "world",
    };

    // act
    const result = replaceAllPlaceholderWithEnv(code, env);

    // assert
    expect(result).toMatchInlineSnapshot(`
      "
            const hello = \\"{\\"HELLO\\":\\"world\\"}\\".HELLO;
          "
    `);
  });
});