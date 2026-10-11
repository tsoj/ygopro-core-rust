//! Myutant ST-46
use super::prelude::*;

pub const ID: u32 = 8200556;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_MYUTANT_BEAST, CARD_MYUTANT_MIST, CARD_MYUTANT_ARSENAL],
    listed_series: &[SET_MYUTANT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If Normal or Special Summoned, add 1 "Myutant" Spell/Trap
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Special Summon 1 monster from hand or deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.spcost)
    effect::set_cost(d, e3, Some(cost_target(spcost)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MYUTANT) and (c:IsSpell() or c:IsTrap()) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MYUTANT as u16]) && (card::is_spell(d, c) || card::is_trap(d, c)) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.getspcode(c)`
fn getspcode(d: &mut Duel, c: CardId) -> i64 {
    // local type=c:GetOriginalType()
    let ty = card::get_original_type(d, c);
    // if type&TYPE_MONSTER==TYPE_MONSTER then return CARD_MYUTANT_BEAST end
    if ty & TYPE_MONSTER == TYPE_MONSTER {
        return CARD_MYUTANT_BEAST as i64;
    }
    // if type&TYPE_SPELL==TYPE_SPELL then return CARD_MYUTANT_MIST end
    if ty & TYPE_SPELL == TYPE_SPELL {
        return CARD_MYUTANT_MIST as i64;
    }
    // return type&TYPE_TRAP==TYPE_TRAP and CARD_MYUTANT_ARSENAL or -1
    if ty & TYPE_TRAP == TYPE_TRAP {
        CARD_MYUTANT_ARSENAL as i64
    } else {
        -1
    }
}
/// `s.spcostfilter(c,e,tp,ft)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToRemoveAsCost() and (ft>0 or Duel.GetMZoneCount(tp,c)>0)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,c,e,tp,s.getspcode(c))
    (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
        && (ft > 0 || duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
        && {
            let code = getspcode(d, c) as u32;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, c.into())
        }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetMZoneCount(tp,c)
    let ft = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0;
    // if chk==0 then return c:IsReleasable()
    //     and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,c,e,tp,ft) end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, a.e, tp, ft)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Release(c,REASON_COST)
    duel::release(d, c, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,1,c,e,tp,ft)
    let rg = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, a.e, tp, ft)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // e:SetLabel(s.getspcode(rg:GetFirst()))
    let first = group::get_first(d, &rg).nil();
    let code = getspcode(d, first);
    effect::set_label(d, a.e, &[code]);
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 or not code then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp,code)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, code as u32)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
