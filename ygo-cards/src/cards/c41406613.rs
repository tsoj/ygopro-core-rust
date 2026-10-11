//! Reign-Beaux, Overking of Dark World
//! 暗黒界の魔神王 レイン
use super::prelude::*;

pub const ID: u32 = 41406613;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 Level 5 or higher "Dark World" monster/Special Summon 1 Level 4 or lower "Dark World" monster if discarded by opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcfilter(c)`
fn spcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DARK_WORLD) and c:IsLevelBelow(7) and c:IsAbleToHandAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && card::is_level_below(d, c, 7) && card::is_able_to_hand_as_cost(d, c)
}
/// `s.chkf1` (`aux.ChkfMMZ(1)` used as the rescon of `aux.SelectUnselectGroup`)
fn chkf1_rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    aux::chkf_mmz(1)(d, sg, e, tp, mg, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let e = a.e(0);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    // for _,te in ipairs(eff) do
    //     local op=te:GetOperation()
    //     if not op or op(e,c) then return false end
    // end
    for te in card::get_card_effect(d, c, EFFECT_NECRO_VALLEY) {
        match effect::get_operation(d, te) {
            None => return false,
            Some(f) => {
                if f.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    matches!(
        aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf1_rescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local rg=Duel.GetMatchingGroup(s.spcfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_RTOHAND,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf1_rescon), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, a.e(0)).as_group().cloned() else { return };
    // Duel.SendtoHand(g,nil,REASON_COST)
    duel::send_to_hand(d, g, None, REASON_COST, None);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if rp==1-tp and c:IsPreviousControler(tp) then
    if a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
    // return c:IsPreviousLocation(LOCATION_HAND) and (r&REASON_EFFECT+REASON_DISCARD)==REASON_EFFECT+REASON_DISCARD
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_EFFECT + REASON_DISCARD)) == REASON_EFFECT + REASON_DISCARD
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_WORLD) and not c:IsCode(id) and c:IsLevelAbove(5) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && !card::is_code(d, c, &[ID]) && card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // local opp_chk=e:GetLabel()
    let opp_chk = effect::get_label(d, a.e);
    // if opp_chk==1 then
    if opp_chk == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    } else {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARK_WORLD) and c:IsLevelBelow(4) and
    //     ((Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    //     or (Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)))
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16])
        && card::is_level_below(d, c, 4)
        && ((duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
            || (duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // local opp_chk=e:GetLabel()
    let opp_chk = effect::get_label(d, a.e);
    // if opp_chk==0 then return end
    if opp_chk == 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    let e = a.e;
    let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // if #sg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if sg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=sg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None);
    // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let sel_op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // local target_player=op==1 and tp or 1-tp
    let target_player = if sel_op == Some(1) { tp } else { 1 - tp };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SpecialSummon(sc,0,tp,target_player,false,false,POS_FACEUP)
    duel::special_summon(d, sc, 0, tp, target_player, false, false, POS_FACEUP as u8, None);
}
