import { useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "antd";

export function ImagePicker() {
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const pickImage = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpg", "jpeg"],
          },
        ],
      });
      if (typeof selected === "string") {
        const url = convertFileSrc(selected);
        setImageUrl(url);
      }
    } catch (e) {}
  };

  return (
    <div>
      <Button color={"primary"} onClick={pickImage}>
        选择图片
      </Button>
      <div
        style={{
          marginTop: 20,
          width: 300,
          height: 300,
          border: "1px solid #ccc",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          overflow: "hidden",
        }}
      >
        {imageUrl ? (
          <img
            src={imageUrl}
            alt="preview"
            style={{
              maxWidth: "100%",
              maxHeight: "100%",
              objectFit: "contain",
            }}
          />
        ) : (
          <span>暂无图片</span>
        )}
      </div>
    </div>
  );
}
