//! Lycoris Lilyreaper
use super::prelude::*;

pub const ID: u32 = 81916745;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1a, CATEGORY_RECOVER);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1a, EVENT_CUSTOM + ID);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.lptg)
    effect::set_target(d, e1a, Some(cost_target(lptg)));
    // e1a:SetOperation(s.lpop)
    effect::set_operation(d, e1a, Some(operation(lpop)));
    // e1a:SetLabelObject(Group.CreateGroup())
    effect::set_label_object(d, e1a, Value::Group(CardSet::new()));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1b, EVENT_TO_GRAVE);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetLabelObject(e1a)
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    // e1b:SetOperation(s.regop)
    effect::set_operation(d, e1b, Some(operation(regop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r&REASON_EFFECT==REASON_EFFECT and ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.lptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.regfilter,nil,tp)
    let base = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &base, &|d, c| regfilter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 0);
    true
}
// function s.lpop(e,tp,eg,ep,ev,re,r,rp)
fn lpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sc=g:GetFirst()
    let mut sc = group::get_first(d, &g);
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
        // sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        sc = group::get_first(d, &sel);
    }
    // if sc then
    if let Some(sc) = sc {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // Duel.Recover(tp,sc:GetAttack(),REASON_EFFECT)
        let atk = card::get_attack(d, sc);
        duel::recover(d, tp, atk as i64, REASON_EFFECT, false, None);
    }
}
// function s.regfilter(c,tp)
fn regfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:GetAttack()>0 and c:IsMonster()
    card::is_controler(d, c, 1 - tp) && card::is_previous_location(d, c, LOCATION_MZONE) && card::get_attack(d, c) > 0 && card::is_monster(d, c)
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=eg:Filter(s.regfilter,nil,tp)
    let eg = a.eg();
    let tg = group::filter(d, &eg, &|d, c| regfilter(d, c, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1a, Value::Group(g));
        // if Duel.GetFlagEffect(tp,id)==0 then
        if duel::get_flag_effect(d, tp, ID) == 0 {
            // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
            // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
            let h = effect::get_handler(d, a.e).nil();
            duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
        }
    }
}
// function s.tgfilter(c,lp)
fn tgfilter(d: &mut Duel, c: CardId, lp: i32) -> bool {
    // return c:IsAttackBelow(lp) and c:IsFaceup() and c:IsAbleToGrave()
    card::is_attack_below(d, c, lp) && card::is_faceup(d, c) && card::is_able_to_grave(d, c, None)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local lp=math.abs(Duel.GetLP(tp)-Duel.GetLP(1-tp))
        let lp = (duel::get_lp(d, tp) - duel::get_lp(d, 1 - tp)).abs();
        // return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,lp)
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, lp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local lp=math.abs(Duel.GetLP(tp)-Duel.GetLP(1-tp))
    let lp = (duel::get_lp(d, tp) - duel::get_lp(d, 1 - tp)).abs();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,lp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, lp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) return r&REASON_EFFECT==REASON_EFFECT and ep==1-tp end
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = d;
    (a.r & REASON_EFFECT) == REASON_EFFECT && a.ep == 1 - a.tp
}
