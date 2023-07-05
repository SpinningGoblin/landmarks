import { Box, Button, Chip, Stack, TextField } from "@mui/material";
import { FC, useState } from "react";

export interface TextListProps {
  valueAdded: (text: string) => void;
  valueRemoved: (text: string) => void;
  values: string[];
  textFieldLabel: string;
  addButtonText: string;
}

export const TextListEditor: FC<TextListProps> = ({
  valueAdded,
  valueRemoved,
  values,
  textFieldLabel,
  addButtonText,
}) => {
  const [current, setCurrent] = useState("");

  return (
    <Box>
      <Stack direction="row">
        <TextField
          label={textFieldLabel}
          onChange={(event) => setCurrent(event.currentTarget.value)}
          value={current}
        />
        <Button
          onClick={() => {
            const found = values.find((val) => val === current);
            if (!found) {
              valueAdded(current.slice());
            }
            setCurrent("");
          }}
        >
          {addButtonText}
        </Button>
      </Stack>
      {values.map((val, index) => (
        <Chip key={index} label={val} onDelete={() => valueRemoved(val)} />
      ))}
    </Box>
  );
};
