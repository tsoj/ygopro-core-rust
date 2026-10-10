//! Steel Scorpion
use super::prelude::*;

pub const ID: u32 = 13599884;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e2, ID);
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_condition(d, e2, Some(condition(resetcon)));
    effect::set_operation(d, e2, Some(operation(resetop)));
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_condition(d, e3, Some(condition(endcond)));
    effect::set_operation(d, e3, Some(operation(endop)));
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_label_object(d, e3, Value::Effect(e2));
    // Duel.RegisterEffect(e3,0)
    duel::register_effect(d, e3, 0);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // return e:GetHandler()==Duel.GetAttackTarget() and not Duel.GetAttacker():IsRace(RACE_MACHINE)
    Some(c) == duel::get_attack_target(d) && !card::is_race(d, duel::get_attacker(d).nil(), RACE_MACHINE)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    // if tc and tc:IsRelateToBattle() then
    if let Some(tc) = duel::get_attacker(d) {
        if card::is_relate_to_battle(d, tc) {
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
            effect::set_owner_player(d, e1, tp);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // e:GetLabelObject():AddCard(tc)
            let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
            group::add_card(&mut g, &CardSet::from_card(tc));
            effect::set_label_object(d, a.e, Value::Group(g));
        }
    }
}
/// `s.rfilter(c,e)`
fn rfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // if not c:IsLocation(LOCATION_MZONE) then return true end
    if !card::is_location(d, c, LOCATION_MZONE) {
        return true;
    }
    // local eff={c:GetCardEffect(id)}
    // for _,te in ipairs(eff) do
    for te in card::get_card_effect(d, c, ID) {
        // if te:GetLabelObject()==e then return false end
        if effect::get_label_object(d, te).as_effect() == Some(e) {
            return false;
        }
    }
    // return true
    true
}
/// `s.resetcon(e)`
fn resetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
    // local rg=g:Filter(s.rfilter,nil,e:GetLabelObject())
    let rg = group::filter(d, &g, &|d, c| rfilter(d, c, e1), NO_EX);
    // g:Sub(rg)
    group::remove_card(&mut g, &rg);
    // Groups are values here: store the changed group again in e1 (Lua changes the shared group).
    effect::set_label_object(d, e1, Value::Group(g.clone()));
    // return #g>0
    !g.is_empty()
}
/// `s.resetop(e,tp,eg,ep,ev,re,r,rp)`
fn resetop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local g=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let g = effect::get_label_object(d, e1).as_group().cloned().nil();
    // local sg=g:Select(tp,1,1,nil)
    let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // s.desop(sg:GetFirst(),e:GetLabelObject(),e:GetHandler(),false)
    let tc = group::get_first(d, &sg).nil();
    desop(d, tc, e1, c, false);
}
/// `s.endcond`: `#e:GetLabelObject():GetLabelObject():GetLabelObject()>0`
fn endcond(d: &mut Duel, a: &EffArgs) -> bool {
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    let g = effect::get_label_object(d, e1).as_group().cloned().nil();
    !g.is_empty()
}
/// `s.endop(e,tp,eg,ep,ev,re,r,rp)`
fn endop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local g=e:GetLabelObject():GetLabelObject():GetLabelObject()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    let g = effect::get_label_object(d, e1).as_group().cloned().nil();
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // s.desop(tc,e:GetLabelObject():GetLabelObject(),e:GetHandler(),true)
        desop(d, tc, e1, c, true);
    }
}
/// `s.desop(tc,e,c,check)`
fn desop(d: &mut Duel, tc: CardId, e: EffectId, c: CardId, check: bool) {
    // local eff={tc:GetCardEffect(id)}
    // for _,te in ipairs(eff) do
    for te in card::get_card_effect(d, tc, ID) {
        // if te:GetLabelObject()==e and (not check or te:GetOwnerPlayer()~=Duel.GetTurnPlayer()) then
        if effect::get_label_object(d, te).as_effect() == Some(e) && (!check || effect::get_owner_player(d, te) != duel::get_turn_player(d)) {
            // local ct=te:GetLabel()+1
            let ct = effect::get_label(d, te) + 1;
            // te:SetLabel(ct)
            effect::set_label(d, te, &[ct]);
            // c:SetTurnCounter(ct)
            card::set_turn_counter(d, c, ct as u16);
            // if ct==3 and Duel.Destroy(tc,REASON_EFFECT)>0 then
            if ct == 3 {
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
