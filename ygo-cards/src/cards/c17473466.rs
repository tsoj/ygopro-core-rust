//! Nervedo the Shadebeast Power Patron
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 17473466;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 53589300], listed_series: &[SET_POWER_PATRON, SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // When a monster effect activated by your opponent in response to the activation of your "Power Patron" or "Artmage" monster's effect resolves, you can negate that effect, then destroy this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish (face-down) the top 3 cards of your Deck; destroy this card, and if you do, Special Summon 1 "Nerva the Power Patron of Creation" from your Extra Deck (this is treated as a Fusion Summon)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.Replaceable(s.descost))
    effect::set_cost(d, e2, Some(cost::replaceable(cost_target(descost), None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is added to your Extra Deck face-up: You can Special Summon 1 "Power Patron" or "Artmage" monster from your Deck or face-up Extra Deck, except "Power Patron Shadow Beast Nervedo"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e3, EVENT_TO_DECK);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ch=Chain.GetCurrentLink()-1
    let ch = duel::get_current_chain(d, false) - 1;
    // return ch>0 and ep==1-tp and re:IsMonsterEffect() and not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsChainDisablable(ev) and Chain.IsTriggeringControler(ch,tp)
    //     and Chain.IsTriggeringType(ch,TYPE_MONSTER) and Chain.IsTriggeringSetcode(ch,{SET_POWER_PATRON,SET_ARTMAGE})
    if ch <= 0 {
        return false;
    }
    let ch = ch as u8;
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_chain_disablable(d, a.ev as u8)
        && chain::is_triggering(d, ch, chain::Prop::Controler, &[a.tp as i64]).truthy()
        && chain::is_triggering(d, ch, chain::Prop::Type, &[TYPE_MONSTER as i64]).truthy()
        && chain::is_triggering(d, ch, chain::Prop::SetCard, &[SET_POWER_PATRON as i64, SET_ARTMAGE as i64]).truthy()
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.SelectEffectYesNo(tp,c,aux.Stringid(id,2)) then return end
    if !duel::select_effect_yes_no(d, a.tp, c, Some(aux::stringid(ID, 2))) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // if Duel.NegateEffect(ev) then
    if duel::negate_effect(d, a.ev as u8) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, a.tp, 3);
    // if chk==0 then return g:IsExists(Card.IsAbleToRemoveAsCost,3,nil,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return group::is_exists(d, &g, &|d, c| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8)), 3, NO_EX);
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
/// `s.nervaspfilter(c,e,tp,hc)`
fn nervaspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: Option<CardId>) -> bool {
    // return c:IsCode(53589300) and Duel.GetLocationCountFromEx(tp,tp,hc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false)
    if !card::is_code(d, c, &[53589300]) {
        return false;
    }
    let ex = match hc {
        Some(h) => h.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.nervaspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| nervaspfilter(d, x, a.e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.Destroy(c,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.nervaspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| nervaspfilter(d, x, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let sc = match group::get_first(d, &sel) {
        Some(sc) => sc,
        None => return,
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, sc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `s.spcon(e)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFaceup()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_faceup(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsSetCard({SET_POWER_PATRON,SET_ARTMAGE}) or c:IsCode(id) or not c:IsCanBeSpecialSummoned(e,0,tp,false,false) then return false end
    if !card::is_set_card(d, c, &[SET_POWER_PATRON as u16, SET_ARTMAGE as u16])
        || card::is_code(d, c, &[ID])
        || !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    } else {
        // return c:IsFaceup() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        card::is_faceup(d, c) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
