import {
  Button,
  Chip,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import { FC, useCallback, useState } from "react";
import { useAddBiome, useRemoveBiome } from "../hooks/landmarks";
import { useBiomes } from "../hooks/minecraft";

export interface LandmarkBiomesProps {
  landmarkId?: string;
  landmarkBiomes: string[];
  allowEditing: boolean;
}

export const LandmarkBiomes: FC<LandmarkBiomesProps> = ({
  landmarkBiomes,
  allowEditing,
  landmarkId,
}) => {
  const [newBiome, setNewBiome] = useState("");

  const addBiomeCallback = useCallback(() => {
    setNewBiome("");
  }, [setNewBiome]);

  const { removeBiome } = useRemoveBiome(landmarkId);
  const { addBiome } = useAddBiome(addBiomeCallback, landmarkId);
  const { biomes } = useBiomes();

  const biomesSaving = removeBiome.isPending || addBiome.isPending;

  return (
    <>
      {biomesSaving && (
        <Typography variant="subtitle1">Saving Biomes</Typography>
      )}
      {!biomesSaving && (
        <Stack direction="row" spacing={1} alignItems={"center"}>
          <Typography variant="subtitle1">Biomes</Typography>
          <Stack
            direction="row"
            spacing={0.5}
            alignItems="center"
            flexWrap="wrap"
          >
            {landmarkBiomes.map((biome, index) => (
              <Chip
                key={index}
                label={biome.replaceAll("_", " ")}
                color="primary"
                onDelete={
                  allowEditing ? () => removeBiome.mutate(biome) : undefined
                }
              />
            ))}
            {allowEditing && (
              <>
                <FormControl>
                  <InputLabel>Biomes</InputLabel>
                  <Select
                    label="Biomes"
                    value={newBiome}
                    onChange={(event) => setNewBiome(event.target.value)}
                    style={{ minWidth: "20em" }}
                    size="small"
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
                <Button
                  onClick={() => {
                    if (!newBiome) {
                      return;
                    }
                    addBiome.mutate(newBiome);
                  }}
                >
                  + Add Biome
                </Button>
              </>
            )}
          </Stack>
        </Stack>
      )}
    </>
  );
};
