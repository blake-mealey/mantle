"use strict";
var __defProp = Object.defineProperty;
var __defProps = Object.defineProperties;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropDescs = Object.getOwnPropertyDescriptors;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getOwnPropSymbols = Object.getOwnPropertySymbols;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __propIsEnum = Object.prototype.propertyIsEnumerable;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __spreadValues = (a, b) => {
  for (var prop in b || (b = {}))
    if (__hasOwnProp.call(b, prop))
      __defNormalProp(a, prop, b[prop]);
  if (__getOwnPropSymbols)
    for (var prop of __getOwnPropSymbols(b)) {
      if (__propIsEnum.call(b, prop))
        __defNormalProp(a, prop, b[prop]);
    }
  return a;
};
var __spreadProps = (a, b) => __defProps(a, __getOwnPropDescs(b));
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);
var __async = (__this, __arguments, generator) => {
  return new Promise((resolve, reject) => {
    var fulfilled = (value) => {
      try {
        step(generator.next(value));
      } catch (e) {
        reject(e);
      }
    };
    var rejected = (value) => {
      try {
        step(generator.throw(value));
      } catch (e) {
        reject(e);
      }
    };
    var step = (x) => x.done ? resolve(x.value) : Promise.resolve(x.value).then(fulfilled, rejected);
    step((generator = generator.apply(__this, __arguments)).next());
  });
};

// src/index.ts
var src_exports = {};
__export(src_exports, {
  flattenSchemaProperties: () => flattenSchemaProperties,
  getSchemas: () => getSchemas,
  isDefined: () => isDefined,
  processSchema: () => processSchema,
  translateToNextra: () => translateToNextra
});
module.exports = __toCommonJS(src_exports);

// src/schemas.ts
var import_rest = require("@octokit/rest");

// src/is-defined.ts
function isDefined(value) {
  return value !== void 0 && value !== null;
}

// src/remark-plugins/translate-to-nextra.ts
var import_unist_builder = require("unist-builder");
var import_unist_util_visit = require("unist-util-visit");
var admonitionTypeToCalloutType = {
  note: "default",
  tip: "default",
  info: "info",
  caution: "warning",
  danger: "error"
};
var translateToNextra = function() {
  return (tree, _file, done) => {
    (0, import_unist_util_visit.visit)(tree, [{ type: "code" }], (node) => {
      const codeNode = node;
      if (codeNode.lang === "yml") {
        codeNode.lang = "yaml";
      }
      if (codeNode.meta) {
        codeNode.meta = codeNode.meta.replace(/title="(.*)"/, 'filename="$1"');
      }
    });
    (0, import_unist_util_visit.visit)(tree, [{ type: "paragraph" }], (node, index, parent) => {
      var _a;
      const paragraphNode = node;
      const firstChild = paragraphNode.children[0];
      if (firstChild == null ? void 0 : firstChild.value.startsWith(":::")) {
        const match = firstChild.value.match(/^:::(\w*)\s*\n/);
        const type = match == null ? void 0 : match[1];
        firstChild.value = firstChild == null ? void 0 : firstChild.value.replace(/^:::.*\n/, "");
        const lastChild = paragraphNode.children[paragraphNode.children.length - 1];
        lastChild.value = lastChild.value.split("\n").slice(0, -1).join("\n");
        const calloutType = (_a = admonitionTypeToCalloutType[type != null ? type : "note"]) != null ? _a : "default";
        const callout = (0, import_unist_builder.u)("mdxJsxFlowElement", {
          name: "Callout",
          attributes: [
            { type: "mdxJsxAttribute", name: "type", value: calloutType }
          ],
          children: paragraphNode.children,
          data: { _mdxExplicitJsx: true }
        });
        parent.children.splice(index, 1, callout);
      }
    });
    done();
  };
};

// src/flatten-schema-properties.ts
function flattenSchemaProperties(compileMdx, schema, parentId) {
  return __async(this, null, function* () {
    var _a;
    if (schema["x-skip-properties"]) {
      return [];
    }
    let properties = [];
    const requiredProps = (_a = schema.required) != null ? _a : [];
    if (isType(schema, "object")) {
      if (schema.properties) {
        for (const [id, definition] of Object.entries(schema.properties)) {
          if (typeof definition === "boolean") {
            continue;
          }
          const formattedId = formatId(id, parentId);
          properties.push({
            id: formattedId,
            level: getLevel(formattedId),
            required: requiredProps.includes(id),
            compiledContent: definition.description ? (yield compileMdx(definition.description, {
              mdxOptions: { remarkPlugins: [translateToNextra] }
            })).result : null,
            propertyType: getSchemaPropertyType(definition),
            default: definition.default === void 0 ? null : { value: definition.default }
          });
          properties.push(
            ...yield flattenSchemaProperties(
              compileMdx,
              definition,
              formattedId
            )
          );
        }
      }
      if (schema.additionalProperties && typeof schema.additionalProperties !== "boolean") {
        properties.push(
          ...yield flattenSchemaProperties(
            compileMdx,
            schema.additionalProperties,
            formatId("<label>", parentId)
          )
        );
      }
    } else if (isType(schema, "array")) {
      const items = Array.isArray(schema.items) ? schema.items : [schema.items];
      for (const definition of items) {
        if (!definition || typeof definition === "boolean") {
          continue;
        }
        properties.push(
          ...yield flattenSchemaProperties(
            compileMdx,
            definition,
            formatId("*", parentId)
          )
        );
      }
    } else if (schema.oneOf) {
      for (const definition of schema.oneOf) {
        if (typeof definition === "boolean") {
          continue;
        }
        properties.push(
          ...yield flattenSchemaProperties(compileMdx, definition, parentId)
        );
      }
    } else if (schema.anyOf) {
      for (const definition of schema.anyOf) {
        if (typeof definition === "boolean") {
          continue;
        }
        properties.push(
          ...yield flattenSchemaProperties(compileMdx, definition, parentId)
        );
      }
    } else {
      if (schema.type !== "string" && schema.type !== "integer" && schema.type !== "boolean" && schema.type !== "number")
        console.log(schema.type, Object.keys(schema));
    }
    return properties;
  });
}
function isType(schema, type) {
  if (!schema.type) {
    return false;
  }
  if (Array.isArray(schema.type)) {
    return schema.type.includes(type);
  } else {
    return schema.type === type;
  }
}
function getValuePropertyType(type) {
  if (typeof type === "object") {
    if (Array.isArray(type)) {
      const item = type[0];
      return {
        type: "array",
        valueType: item ? getValuePropertyType(item) : { type: "primitive", value: "unknown" }
      };
    }
    return {
      type: "primitive",
      value: "object"
    };
  }
  return {
    type: "primitive",
    value: type.toString()
  };
}
function getSchemaPropertyType(schema) {
  var _a, _b;
  let type;
  if (Array.isArray(schema.type)) {
    type = schema.type.find((x) => x !== "null");
  } else {
    type = schema.type;
  }
  if (schema.enum) {
    let values = schema.enum;
    if (type === "string") {
      values = values.map((value) => `'${value}'`);
    }
    return {
      type: "enum",
      values: values.map(getValuePropertyType)
    };
  }
  if (type === "number" || type === "integer") {
    return {
      type: "primitive",
      value: (_a = schema.format) != null ? _a : type
    };
  }
  if (type === "array" && schema.items && typeof schema.items !== "boolean" && !Array.isArray(schema.items)) {
    return {
      type: "array",
      valueType: getSchemaPropertyType(schema.items)
    };
  }
  if (type === "object") {
    if (schema.additionalProperties) {
      return {
        type: "dictionary",
        valueType: typeof schema.additionalProperties !== "boolean" ? getSchemaPropertyType(schema.additionalProperties) : { type: "primitive", value: "unknown" }
      };
    }
    return {
      type: "object",
      properties: Object.entries((_b = schema.properties) != null ? _b : {}).map(
        ([name, property]) => typeof property !== "boolean" ? [name, getSchemaPropertyType(property)] : void 0
      ).filter(isDefined)
    };
  }
  if (schema.oneOf) {
    const oneOf = {
      type: "oneOf",
      values: schema.oneOf.filter(
        (definition) => typeof definition !== "boolean"
      ).map(getSchemaPropertyType)
    };
    if (oneOf.values.length === 1) {
      return oneOf.values[0];
    }
    return oneOf;
  }
  if (schema.anyOf) {
    const anyOf = {
      type: "anyOf",
      values: schema.anyOf.filter(
        (definition) => typeof definition !== "boolean"
      ).map(getSchemaPropertyType)
    };
    if (anyOf.values.length === 1) {
      return anyOf.values[0];
    }
    return anyOf;
  }
  return {
    type: "primitive",
    value: type != null ? type : "unknown"
  };
}
function formatId(id, parentId) {
  return (parentId ? `${parentId}.` : "") + id;
}
function getLevel(id) {
  return id.includes(".") ? 4 : 3;
}

// src/schemas.ts
var schemasCache;
function getSchemas() {
  return __async(this, null, function* () {
    var _a;
    if (schemasCache) {
      return schemasCache;
    }
    const client = new import_rest.Octokit({ auth: process.env.GITHUB_TOKEN });
    const repoParams = { owner: "blake-mealey", repo: "mantle" };
    const releases = yield client.paginate(
      client.rest.repos.listReleases,
      repoParams
    );
    const schemas = {};
    yield Promise.all(
      releases.map((release) => __async(this, null, function* () {
        var _a2;
        const asset_id = (_a2 = release.assets.find(
          (asset) => asset.name === "schema.json"
        )) == null ? void 0 : _a2.id;
        if (!asset_id)
          return;
        const response = yield client.rest.repos.getReleaseAsset(__spreadProps(__spreadValues({}, repoParams), {
          asset_id,
          headers: {
            accept: "application/octet-stream"
          }
        }));
        schemas[release.tag_name] = JSON.parse(
          Buffer.from(response.data).toString("utf8")
        );
      }))
    );
    schemasCache = { schemas, latestVersion: (_a = releases[0]) == null ? void 0 : _a.tag_name };
    return schemasCache;
  });
}
function processSchema(_0, _1) {
  return __async(this, arguments, function* (compileMdx, {
    version,
    schema
  }) {
    return {
      version,
      properties: yield flattenSchemaProperties(compileMdx, schema)
    };
  });
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  flattenSchemaProperties,
  getSchemas,
  isDefined,
  processSchema,
  translateToNextra
});
//# sourceMappingURL=index.cjs.map