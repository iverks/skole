#!/usr/bin/bash
#SBATCH --partition=GPUQ
#SBATCH --account=share-nv-fys
#SBATCH --time=00:30:00
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=1
#SBATCH --gres=gpu:1
#SBATCH --job-name="mumax3"
#SBATCH --output=geometry.%j.out

cd $SLURM_SUBMIT_DIR
module purge
module load mumax3/10-gcccuda-2020b
mumax3 run.mx3
mumax3-convert -png 2048.out/*.ovf
