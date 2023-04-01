import { FC } from "react";
import { Box } from "@chakra-ui/react";

interface DiagramCanvasProps {
  id: string;
  isActive: boolean;
}

export const DiagramCanvas: FC<DiagramCanvasProps> = ({ id, isActive }) => {
  const display = isActive ? "block" : "none";

  return <Box as={"canvas"} display={display} id={id}></Box>;
};
