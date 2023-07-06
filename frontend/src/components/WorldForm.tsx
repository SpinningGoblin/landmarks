import { FC, useState } from "react";
import { EditingWorld } from "../models/EditingWorld";
import { WorldRecord } from "../models/WorldRecord";
import {
  Box,
  Button,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { Platform } from "../api/Platform";
import { TextListEditor } from "./TextListEditor";

export interface WorldFormProps {
  world: EditingWorld;
  onFormSubmit: (record: WorldRecord) => void;
}

export const WorldForm: FC<WorldFormProps> = ({ world, onFormSubmit }) => {
  const [seed, setSeed] = useState(world.seed);
  const [name, setName] = useState(world.name);
  const [platform, setPlatform] = useState<Platform | "">(world.platform);
  const [tags, setTags] = useState(world.tags);
  const [notes, setNotes] = useState(world.notes);

  const handleDimensionChange = (event: SelectChangeEvent) => {
    setPlatform(event.target.value as Platform);
  };

  const handleSubmitWorld = () => {
    const record: WorldRecord = {
      name: name !== "" ? name : undefined,
      seed,
      tags,
      notes,
      platform: platform as Platform,
    };
    onFormSubmit(record);
  };

  const validWorld = !!seed && !!platform;

  return (
    <Stack spacing={1}>
      <Box>
        <TextField
          label="World Seed"
          onChange={(event) => setSeed(event.currentTarget.value)}
          value={seed}
          error={!seed}
        />
      </Box>
      <Box>
        <TextField
          label="World Name"
          onChange={(event) => setName(event.currentTarget.value)}
          value={name}
        />
      </Box>
      <FormControl>
        <InputLabel>Platform</InputLabel>
        <Select
          error={!platform}
          label="Platform"
          value={platform}
          onChange={handleDimensionChange}
          style={{ minWidth: "20em" }}
        >
          <MenuItem value={"bedrock"}>
            <Typography>Bedrock</Typography>
          </MenuItem>
          <MenuItem value={"java"}>
            <Typography>Java</Typography>
          </MenuItem>
        </Select>
      </FormControl>
      <TextListEditor
        addButtonText="+ Add Tag"
        textFieldLabel="New Tag"
        values={tags}
        valueAdded={(value) => setTags((current) => [...current, value])}
        valueRemoved={(value) =>
          setTags((current) => {
            const index = current.findIndex((val) => val === value);
            if (index >= 0) {
              const copy = current.slice();
              copy.splice(index, 1);
              return copy;
            } else {
              return current;
            }
          })
        }
      />
      <Box>
        <TextField
          label="Notes"
          onChange={(event) => setNotes(event.currentTarget.value)}
          value={notes}
        />
      </Box>
      <Button
        variant="contained"
        disabled={!validWorld}
        onClick={handleSubmitWorld}
      >
        Save
      </Button>
    </Stack>
  );
};
