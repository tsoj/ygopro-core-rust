//! Zone Eater
use super::prelude::*;

pub const ID: u32 = 86100785;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCode(1082946)
    effect::set_code(d, e2, 1082946);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(s.resetcon)
    effect::set_condition(d, e2, Some(condition(resetcon)));
    // e2:SetOperation(s.resetop)
    effect::set_operation(d, e2, Some(operation(resetop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetCondition(function(e)return #e:GetLabelObject():GetLabelObject():GetLabelObject()>0 end)
    effect::set_condition(d, e3, Some(condition(e3cond)));
    // e3:SetOperation(s.endop)
    effect::set_operation(d, e3, Some(operation(endop)));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // Duel.RegisterEffect(e3,0)
    duel::register_effect(d, e3, 0);
}
/// `e:GetLabelObject():GetLabelObject()` style: the group stored in the label object of `e`.
fn label_group(d: &Duel, e: EffectId) -> CardSet {
    effect::get_label_object(d, e).as_group().cloned().nil()
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c==Duel.GetAttacker() and bc and bc:IsRelateToBattle() and bc:IsFaceup()
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    match bc {
        Some(bc) => card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc),
        None => false,
    }
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(id)
            effect::set_code(d, e1, ID);
            // e1:SetLabelObject(e)
            effect::set_label_object(d, e1, Value::Effect(a.e));
            // e1:SetLabel(0)
            effect::set_label(d, e1, &[0]);
            // e1:SetOwnerPlayer(tp)
            effect::set_owner_player(d, e1, a.tp);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // bc:RegisterEffect(e1)
            card::register_effect(d, bc, e1, false);
            // e:GetLabelObject():AddCard(bc)
            let mut g = label_group(d, a.e);
            group::add_card(&mut g, &CardSet::from_card(bc));
            effect::set_label_object(d, a.e, Value::Group(g));
        }
    }
}
fn rfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // if not c:IsLocation(LOCATION_MZONE) then return true end
    if !card::is_location(d, c, LOCATION_MZONE) {
        return true;
    }
    // local eff={c:GetCardEffect(id)}
    let eff = card::get_card_effect(d, c, ID);
    // for _,te in ipairs(eff) do
    for te in eff {
        // if te:GetLabelObject()==e then return false end
        if effect::get_label_object(d, te).as_effect() == Some(e) {
            return false;
        }
    }
    // return true
    true
}
fn resetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let mut g = label_group(d, e1);
    // local rg=g:Filter(s.rfilter,nil,e:GetLabelObject())
    let rg = group::filter(d, &g, &|d, c| rfilter(d, c, e1), NO_EX);
    // g:Sub(rg)
    group::remove_card(&mut g, &rg);
    // the stored group is the one `g` refers to in Lua: store the changed copy again
    effect::set_label_object(d, e1, Value::Group(g.clone()));
    // return #g>0
    !g.is_empty()
}
fn resetop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let g = label_group(d, e1);
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // s.desop(sg:GetFirst(),e:GetLabelObject(),e:GetHandler())
    let tc = group::get_first(d, &sg).nil();
    let c = effect::get_handler(d, a.e).nil();
    desop(d, tc, e1, c);
}
fn desop(d: &mut Duel, tc: CardId, e: EffectId, c: CardId) {
    // local eff={tc:GetCardEffect(id)}
    let eff = card::get_card_effect(d, tc, ID);
    // for _,te in ipairs(eff) do
    for te in eff {
        // if te:GetLabelObject()==e then
        if effect::get_label_object(d, te).as_effect() == Some(e) {
            // local ct=te:GetLabel()+1
            let ct = effect::get_label(d, te) + 1;
            // te:SetLabel(ct)
            effect::set_label(d, te, &[ct]);
            // c:SetTurnCounter(ct)
            card::set_turn_counter(d, c, ct as u16);
            // if ct==5 and Duel.Destroy(tc,REASON_EFFECT)>0 then
            if ct == 5 {
                duel::destroy(d, tc, REASON_EFFECT, None, None);
                //to be added if andre's PRs are merged
                //tc:SetReasonPlayer(te:GetOwnerPlayer())
                //tc:SetReasonEffect(te)
            }
        }
    }
}
fn endop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject():GetLabelObject():GetLabelObject()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    let g = label_group(d, e1);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g:Clone()) do
    // (ygoai override lua_fixes/c86100785.lua: the original iterated g itself, which s.resetcon changes during s.desop's
    // destroy; group::iter is a copy)
    for tc in group::iter(d, &g) {
        // s.desop(tc,e:GetLabelObject():GetLabelObject(),e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        desop(d, tc, e1, c);
    }
}
fn e3cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return #e:GetLabelObject():GetLabelObject():GetLabelObject()>0
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    label_group(d, e1).len() > 0
}
