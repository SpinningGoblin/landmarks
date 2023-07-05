import { TextField } from "@mui/material";
import { FC } from "react";

export interface NumberFieldProps {
  value: string;
  setValue: (text: string) => void;
  label: string;
  required?: boolean;
}

export const NumberField: FC<NumberFieldProps> = ({
  value,
  setValue,
  label,
  required,
}) => {
  const error = required && !value;

  return (
    <TextField
      value={value}
      onChange={(event) => {
        if (
          event.currentTarget.value === "" ||
          event.currentTarget.value === "-"
        ) {
          setValue(event.currentTarget.value);
        } else {
          const parsed = parseInt(event.currentTarget.value);
          if (!isNaN(parsed)) {
            setValue(parsed.toString());
          }
        }
      }}
      label={label}
      error={error}
    />
  );
};
