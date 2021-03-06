import { UserConfig } from "vite";
import { mergeManualChunks } from "../merge-manual-chunks";
import { virtualFile } from "../../../../shared";

describe("merge-manual-chunks", () => {
  test("build.rollupOptions.output is undefined", () => {
    // arrange
    const userConfig: UserConfig = {
      base: "/",
      build: {
        lib: false,
        rollupOptions: {
          input: "src/index.ts",
        },
      },
    };

    // act
    const result = mergeManualChunks(userConfig);

    // asesrt
    expect(result).toMatchInlineSnapshot(`
      Object {
        "base": "/",
        "build": Object {
          "lib": false,
          "rollupOptions": Object {
            "input": "src/index.ts",
            "output": Object {
              "manualChunks": Object {
                "import-meta-env": Array [
                  "import-meta-env",
                ],
              },
            },
          },
        },
      }
    `);
  });

  describe("build.rollupOptions.output is array", () => {
    test("build.rollupOptions.output[].manualChunks is undefined", () => {
      // arrange
      const userConfig: UserConfig = {
        base: "/",
        build: {
          lib: false,
          rollupOptions: {
            input: "src/index.ts",
            output: [
              {
                sourcemap: true,
              },
            ],
          },
        },
      };

      // act
      const result = mergeManualChunks(userConfig);

      // asesrt
      expect(result).toMatchInlineSnapshot(`
        Object {
          "base": "/",
          "build": Object {
            "lib": false,
            "rollupOptions": Object {
              "input": "src/index.ts",
              "output": Array [
                Object {
                  "manualChunks": Object {
                    "import-meta-env": Array [
                      "import-meta-env",
                    ],
                  },
                  "sourcemap": true,
                },
              ],
            },
          },
        }
      `);
    });
  });

  describe("build.rollupOptions.output is object", () => {
    test("build.rollupOptions.output.manualChunks is undefined", () => {
      // arrange
      const userConfig: UserConfig = {
        base: "/",
        build: {
          lib: false,
          rollupOptions: {
            input: "src/index.ts",
            output: {
              sourcemap: true,
            },
          },
        },
      };

      // act
      const result = mergeManualChunks(userConfig);

      // asesrt
      expect(result).toMatchInlineSnapshot(`
        Object {
          "base": "/",
          "build": Object {
            "lib": false,
            "rollupOptions": Object {
              "input": "src/index.ts",
              "output": Object {
                "manualChunks": Object {
                  "import-meta-env": Array [
                    "import-meta-env",
                  ],
                },
                "sourcemap": true,
              },
            },
          },
        }
      `);
    });

    test("build.rollupOptions.output.manualChunks is object", () => {
      // arrange
      const userConfig: UserConfig = {
        base: "/",
        build: {
          lib: false,
          rollupOptions: {
            input: "src/index.ts",
            output: {
              manualChunks: {
                foo: ["bar"],
              },
              sourcemap: true,
            },
          },
        },
      };

      // act
      const result = mergeManualChunks(userConfig);

      // asesrt
      expect(result).toMatchInlineSnapshot(`
        Object {
          "base": "/",
          "build": Object {
            "lib": false,
            "rollupOptions": Object {
              "input": "src/index.ts",
              "output": Object {
                "manualChunks": Object {
                  "foo": Array [
                    "bar",
                  ],
                  "import-meta-env": Array [
                    "import-meta-env",
                  ],
                },
                "sourcemap": true,
              },
            },
          },
        }
      `);
    });

    test("build.rollupOptions.output.manualChunks is function", () => {
      // arrange
      const userConfig: UserConfig = {
        base: "/",
        build: {
          lib: false,
          rollupOptions: {
            input: "src/index.ts",
            output: {
              manualChunks: (id) => {
                if (id === "bar") {
                  return "foo";
                }
              },
              sourcemap: true,
            },
          },
        },
      };

      // act
      const result = mergeManualChunks(userConfig);

      // asesrt
      expect(result).toMatchInlineSnapshot(`
        Object {
          "base": "/",
          "build": Object {
            "lib": false,
            "rollupOptions": Object {
              "input": "src/index.ts",
              "output": Object {
                "manualChunks": [Function],
                "sourcemap": true,
              },
            },
          },
        }
      `);
      expect(
        (result as any).build.rollupOptions.output.manualChunks("bar")
      ).toBe("foo");
      expect(
        (result as any).build.rollupOptions.output.manualChunks(virtualFile)
      ).toBe(virtualFile);
      expect(
        (result as any).build.rollupOptions.output.manualChunks(undefined)
      ).toBe(undefined);
    });
  });
});
