//! Ghost Mourner & Moonlit Chill
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 52038441;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // g:KeepAlive() (groups are values in Rust)
    // Negate effects of a Special Summoned monster and inflict damage if it leaves the field this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register summons
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1a:SetRange(LOCATION_ALL)
    effect::set_range(d, e1a, LOCATION_ALL);
    // e1a:SetLabelObject(e1)
    effect::set_label_object(d, e1a, Value::Effect(e1));
    // e1a:SetOperation(s.regop)
    effect::set_operation(d, e1a, Some(operation(regop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsCanBeEffectTarget(e) and c:IsLocation(LOCATION_MZONE) and (c:HasNonZeroAttack() or c:IsNegatableMonster())
    card::is_summon_player(d, c, 1 - tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_MZONE)
        && (card::has_non_zero_attack(d, c) || card::is_negatable_monster(d, c))
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let eg = a.eg();
    // local tg=eg:Filter(s.cfilter,nil,e,tp)
    let tg = group::filter(d, &eg, &|d, c| cfilter(d, c, e, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1, Value::Group(g));
        // if Duel.GetFlagEffect(tp,id)==0 then
        if duel::get_flag_effect(d, tp, ID) == 0 {
            // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
            // Duel.RaiseEvent(eg,EVENT_CUSTOM+id,e,0,tp,tp,0)
            duel::raise_event(d, eg, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
        }
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g=e:GetLabelObject():Filter(s.cfilter,nil,e,tp)
    let lo = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, c| cfilter(d, c, e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=nil
    // if #g>1 then
    let tg = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // tg=g:Select(tp,1,1,nil)
        group::select(d, &g, tp, 1, 1, false, NO_EX).nil()
    }
    // else tg=g end
    else {
        g.clone()
    };
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if tc:IsNegatableMonster() then
    if card::is_negatable_monster(d, tc) {
        // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
    }
    // if tc:IsFaceup() then
    if card::is_faceup(d, tc) {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESET_MSCHANGE|RESET_OVERLAY|RESET_TURN_SET|RESET_PHASE|PHASE_END,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESET_MSCHANGE | RESET_OVERLAY | RESET_TURN_SET | RESET_PHASE | PHASE_END, 0, 1, fid, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_LEAVE_FIELD)
        effect::set_code(d, e1, EVENT_LEAVE_FIELD);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid]);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetOperation(s.leaveop)
        effect::set_operation(d, e1, Some(operation(leaveop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.leaveop(e,tp,eg,ep,ev,re,r,rp)`
fn leaveop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if eg:IsContains(tc) and tc:GetFlagEffectLabel(id)==e:GetLabel() then
    if a.eg.clone().nil().contains(tc) && card::get_flag_effect_label(d, tc, ID).first().copied() == Some(effect::get_label(d, a.e)) {
        // local p=tc:GetPreviousControler()
        let p = card::get_previous_controler(d, tc);
        // if Duel.Damage(p,tc:GetBaseAttack(),REASON_EFFECT)>0 then
        let atk = card::get_base_attack(d, tc) as i64;
        if duel::damage(d, p, atk, REASON_EFFECT, false, None) > 0 {
            // Duel.Hint(HINT_CARD,0,id)
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        }
        // tc:ResetFlagEffect(id)
        card::reset_flag_effect(d, tc, ID);
        // e:Reset()
        effect::reset(d, a.e);
    }
}
