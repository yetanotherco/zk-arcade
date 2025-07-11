import React, { useCallback, useEffect, useRef, useState } from "react";
import { useAccount, useSignMessage } from "wagmi";
import { ConnectKitButton } from "connectkit";

const FormAgreementSubmit = () => {
	const formRef = useRef<HTMLFormElement>(null);
	const [csrfToken, setCsrfToken] = useState("");

	const { address } = useAccount();
	const [signature, setSignature] = useState("");
	const { signMessageAsync } = useSignMessage();

	useEffect(() => {
		const csrfToken =
			document.head
				.querySelector("[name~=csrf-token]")
				?.getAttribute("content") || "";
		setCsrfToken(csrfToken);
	}, []);

	const handleSubmit = useCallback(async () => {
		try {
			const sig = await signMessageAsync({
				message: "I agree with the service policy",
			});
			setSignature(sig);
		} catch (err) {
			alert(`Error signing: ${err}`);
		}
	}, [setSignature, formRef.current]);

	useEffect(() => {
		if (signature) formRef.current?.submit();
	}, [signature]);

	return (
		<>
			<h3 className="pb-2">1. Connect your funded Ethereum wallet</h3>
			<div className="pb-6">
				<ConnectKitButton />
			</div>
			<form ref={formRef} action="/sign" method="post">
				<input type="hidden" name="_csrf_token" value={csrfToken} />
				<input type="hidden" name="address" value={address || ""} />
				<input type="hidden" name="signature" value={signature} />
			</form>
			{address && (
				<>
					<h3 className="pb-2">
						2. Sign your agreement with the service policy
					</h3>
					<button
						onClick={handleSubmit}
						className="default-btn"
						disabled={!address}
					>
						Sign agreement
					</button>
				</>
			)}
		</>
	);
};

export default FormAgreementSubmit;
