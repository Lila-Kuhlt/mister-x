import { Button, DropDown, TextInput } from 'components/InputElements';
import { postCreateTeam } from 'lib/api';
import { TeamKind } from 'lib/bindings';
import { FormEvent, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import { red, pink, lime, cyan, purple } from 'tailwindcss/colors';

export function CreateTeam() {
    const [color, setColor] = useState<string>(purple['500']);
    const [name, setName] = useState<string>('');
    const [kind, setKind] = useState<TeamKind>('Detective');

    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();
    const { t } = useTranslation();

    const colors = [
        red['500'],
        pink['500'],
        lime['500'],
        cyan['500'],
        purple['500'],
    ];

    const onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        setLoading(true);
        postCreateTeam({ color, name, kind })
            .then(() => {
                setLoading(false);
                navigate('/');
            })
            .catch((err) => {
                setLoading(false);
                alert(t(err.response.data));
            });
    };

    return (
        <div
            className="flex h-screen items-center justify-center transition-colors"
            style={{ backgroundColor: color }}>
            <form
                className="container flex w-80 flex-col gap-3 rounded-xl bg-white p-8 shadow-md"
                onSubmit={onSubmit}>
                <h2 className="text-xl font-bold">{t('CreateTeam')}</h2>

                <TextInput onTextChange={setName} trim="all" />

                <DropDown<TeamKind>
                    onItemChange={setKind}
                    items={['Detective', 'MrX', 'Observer']}
                />

                <div className="flex justify-between gap-3">
                    {colors.map((color) => (
                        <div
                            className="h-10 w-10 rounded-md"
                            style={{ backgroundColor: color }}
                            key={color}
                            onClick={() => setColor(color)}
                        />
                    ))}
                </div>

                <Button disabled={loading}>
                    {loading ? (
                        <div className="h-4 w-4 animate-spin rounded-full border-4 border-dashed dark:border-white"></div>
                    ) : (
                        <>{t('CreateTeam')}</>
                    )}
                </Button>
            </form>
        </div>
    );
}
