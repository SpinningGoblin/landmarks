import { FC, useState } from "react";
import { EditingLandmark } from "../models/EditingLandmark";
import { useBiomes, useDimensions } from "../hooks/minecraft";
import {
  Box,
  Button,
  Chip,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { LandmarkRecord } from "../models/LandmarkRecord";
import { NumberField } from "./NumberField";
import { TextListEditor } from "./TextListEditor";

export interface LandmarkFormProps {
  formTitle: string;
  landmark: EditingLandmark;
  onFormSubmit: (landmark: LandmarkRecord) => void;
}

export const LandmarkForm: FC<LandmarkFormProps> = ({
  formTitle,
  landmark,
  onFormSubmit,
}) => {
  const { biomes, isLoading: isBiomesLoading } = useBiomes();
  const { dimensions, isLoading: isDimensionsLoading } = useDimensions();
  const [name, setName] = useState(landmark.name);
  const [x, setX] = useState(landmark.coordinate.x);
  const [y, setY] = useState(landmark.coordinate.y);
  const [z, setZ] = useState(landmark.coordinate.z);
  const [tags, setTags] = useState<string[]>(landmark.tags);
  const [farms, setFarms] = useState<string[]>(landmark.farms);
  const [dimension, setDimension] = useState(landmark.dimension);
  const [selectedBiomes, setSelectedBiomes] = useState<string[]>(
    landmark.biomes,
  );
  const [notes, setNotes] = useState(landmark.notes);

  const handleDimensionChange = (event: SelectChangeEvent) => {
    setDimension(event.target.value as string);
  };

  const handleBiomeChange = (
    event: SelectChangeEvent<typeof selectedBiomes>,
  ) => {
    const {
      target: { value },
    } = event;
    setSelectedBiomes(
      // On autofill we get a stringified value.
      typeof value === "string" ? value.split(",") : value,
    );
  };

  const handleSubmitLandmark = () => {
    const record: LandmarkRecord = {
      name,
      notes,
      biomes: selectedBiomes,
      dimension,
      tags,
      farms,
      coordinate: {
        x: parseInt(x),
        y: parseInt(y),
        z: parseInt(z),
      },
    };

    onFormSubmit(record);
  };

  if (isBiomesLoading || isDimensionsLoading) {
    return <Typography>Loading ...</Typography>;
  }

  const validLandmark = !!name && !!dimension && !!x && !!y && !!z;

  return (
    <Stack spacing={1}>
      <Typography variant="h6">{formTitle}</Typography>
      <Stack spacing={2} alignItems="start" sx={{ paddingLeft: "0.5em" }}>
        <Box>
          <TextField
            label="Landmark Name"
            onChange={(event) => setName(event.currentTarget.value)}
            value={name}
            error={!name}
          />
        </Box>
        <Box>
          <Stack direction="row" spacing={1}>
            <NumberField label="X" required value={x} setValue={setX} />
            <NumberField label="Y" required value={y} setValue={setY} />
            <NumberField label="Z" required value={z} setValue={setZ} />
          </Stack>
        </Box>
        <FormControl>
          <InputLabel>Dimension</InputLabel>
          <Select
            error={!dimension}
            label="Dimension"
            value={dimension}
            onChange={handleDimensionChange}
            style={{ minWidth: "20em" }}
          >
            {dimensions?.map((dim) => (
              <MenuItem key={dim} value={dim}>
                <Typography style={{ textTransform: "capitalize" }}>
                  {dim}
                </Typography>
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <FormControl>
          <InputLabel>Biomes</InputLabel>
          <Select
            label="Biomes"
            multiple
            value={selectedBiomes}
            onChange={handleBiomeChange}
            style={{ minWidth: "20em" }}
            renderValue={(selected) => (
              <Box sx={{ display: "flex", flexWrap: "wrap", gap: 0.5 }}>
                {selected.map((value) => (
                  <Chip
                    key={value}
                    label={value.replaceAll("_", " ")}
                    style={{ textTransform: "capitalize" }}
                  />
                ))}
              </Box>
            )}
          >
            {biomes?.map((b) => (
              <MenuItem key={b} value={b}>
                <Typography style={{ textTransform: "capitalize" }}>
                  {b.replaceAll("_", " ")}
                </Typography>
              </MenuItem>
            ))}
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
        <TextListEditor
          addButtonText="+ Add Farm"
          textFieldLabel="New Farm"
          values={farms}
          valueAdded={(value) => setFarms((current) => [...current, value])}
          valueRemoved={(value) =>
            setFarms((current) => {
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
      </Stack>
      <Button
        variant="contained"
        disabled={!validLandmark}
        onClick={handleSubmitLandmark}
      >
        Save
      </Button>
    </Stack>
  );
};
