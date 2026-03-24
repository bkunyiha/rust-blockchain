import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Zap } from 'lucide-react';
import { useInvoke } from '../../hooks/useInvoke';
import { commands } from '../../lib/commands';
import JsonViewer from '../../components/JsonViewer';
import { useAppStore } from '../../store/useAppStore';

const generateSchema = z.object({
  address: z.string().min(1, 'Address is required'),
  nblocks: z.coerce.number().positive('Number of blocks must be positive').integer(),
  maxtries: z.coerce.number().positive().integer().optional(),
});

type GenerateForm = z.infer<typeof generateSchema>;

function GenerateBlocksPage() {
  const setStatus = useAppStore((state) => state.setStatus);
  const { register, handleSubmit, formState: { errors } } = useForm<GenerateForm>({
    resolver: zodResolver(generateSchema),
  });

  const { data, loading, error, execute } = useInvoke({
    onSuccess: () => {
      setStatus('Blocks generated successfully');
    },
    onError: (err) => {
      setStatus(`Error: ${err}`);
    },
  });

  const onSubmit = async (formData: GenerateForm) => {
    await execute('generate_blocks', {
      address: formData.address,
      nblocks: formData.nblocks,
      maxtries: formData.maxtries,
    });
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold">Generate Blocks</h1>

      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        <div className="rounded-lg border border-slate-700 bg-slate-800 p-6">
          <div className="space-y-4">
            {/* Address */}
            <div>
              <label className="block text-sm font-medium mb-2">
                Miner Address
              </label>
              <input
                type="text"
                {...register('address')}
                placeholder="Bitcoin address for mining rewards..."
                className="w-full rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
              />
              {errors.address && (
                <p className="mt-1 text-sm text-red-400">
                  {errors.address.message}
                </p>
              )}
            </div>

            {/* Number of Blocks */}
            <div>
              <label className="block text-sm font-medium mb-2">
                Number of Blocks
              </label>
              <input
                type="number"
                {...register('nblocks')}
                placeholder="1"
                className="w-full rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
              />
              {errors.nblocks && (
                <p className="mt-1 text-sm text-red-400">
                  {errors.nblocks.message}
                </p>
              )}
            </div>

            {/* Max Tries */}
            <div>
              <label className="block text-sm font-medium mb-2">
                Max Tries (Optional)
              </label>
              <input
                type="number"
                {...register('maxtries')}
                placeholder="1000000"
                className="w-full rounded-lg border border-slate-600 bg-slate-700 px-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
              />
              {errors.maxtries && (
                <p className="mt-1 text-sm text-red-400">
                  {errors.maxtries.message}
                </p>
              )}
            </div>

            <button
              type="submit"
              disabled={loading}
              className="flex items-center gap-2 rounded-lg bg-yellow-500 px-6 py-2 font-medium text-slate-900 hover:bg-yellow-600 disabled:opacity-50"
            >
              <Zap className="h-5 w-5" />
              {loading ? 'Generating...' : 'Generate'}
            </button>
          </div>
        </div>
      </form>

      {error && (
        <div className="rounded-lg border border-red-700 bg-red-900/20 p-8 text-red-200">
          <p>Failed to generate blocks</p>
          <p className="text-sm opacity-75">{error}</p>
        </div>
      )}

      {data && (
        <JsonViewer data={data} title="Generation Results" maxHeight={700} />
      )}
    </div>
  );
}

export default GenerateBlocksPage;
