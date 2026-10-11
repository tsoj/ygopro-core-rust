//! Vaalmonica Invitare
//! ヴァルモニカ・インヴィターレ
use super::prelude::*;

pub const ID: u32 = 38491852;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAALMONICA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.nonpendfilter(c)`
fn nonpendfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAALMONICA) and not c:IsType(TYPE_PENDULUM)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && !card::is_type(d, c, TYPE_PENDULUM)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsMonster() and c:IsType(TYPE_PENDULUM)
    //     and (c:IsAbleToHand() or not c:IsForbidden())
    card::is_set_card(d, c, &[SET_VAALMONICA as u16])
        && card::is_monster(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && (card::is_able_to_hand(d, c, None) || !card::is_forbidden(d, c))
}
/// `s.extrachk(c,sg)`
fn extrachk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAbleToHand() and sg:IsExists(aux.TRUE,1,c)
    card::is_able_to_hand(d, c, None) && group::is_exists(d, sg, &|_, _| true, 1, c.into())
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)>=2 and sg:IsExists(s.extrachk,1,nil,sg)
    let classes = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64);
    let res = classes >= 2 && group::is_exists(d, sg, &|d, c| extrachk(d, c, sg), 1, NO_EX);
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
        // if b1 then return true end
        if b1 {
            return true;
        }
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK, 0, NO_EX);
        // local b2=Duel.IsExistingMatchingCard(s.nonpendfilter,tp,LOCATION_MZONE,0,1,nil)
        //     and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        let b2 = duel::is_existing_matching_card(d, Some(&nonpendfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
        // return b1 or b2
        return b1 || b2;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.nonpendfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    let b2 = duel::is_existing_matching_card(d, Some(&nonpendfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && g.len() >= 2
        && matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).map(|v| v as i64);
    // if op==1 then
    if op == Some(1) {
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
        if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
            // Cannot activate effects of monsters, except "Vaalmonica" monsters
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetValue(s.actfilter)
            effect::set_value_fn(d, e1, lua_fn(actfilter));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        }
        // Special Summon 1 "Vaalmonica" monster from your Deck
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let sc = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sc>0 then
        if !sc.is_empty() {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==2 then
    } else if op == Some(2) {
        // Take 2 "Vaalmonica" Pendulum Monsters from your Deck
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,4))
        let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 4)), None, None, false)
            .unwrap_or_default();
        // if #sg~=2 then return end
        if sg.len() != 2 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local thg=sg:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let thg = group::filter_select(d, &sg, tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        // Duel.SendtoExtraP(sg:Sub(thg),tp,REASON_EFFECT)
        let mut rest = sg.clone();
        group::remove_card(&mut rest, &thg);
        duel::send_to_extra_p(d, rest, Some(tp), REASON_EFFECT, None);
        // Duel.SendtoHand(thg,nil,REASON_EFFECT)
        duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,thg)
        duel::confirm_cards(d, 1 - tp, thg);
    }
}
/// `s.actfilter(e,re,tp)`
fn actfilter(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsOnField() and not rc:IsSetCard(SET_VAALMONICA)
    effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && !card::is_set_card(d, rc, &[SET_VAALMONICA as u16])
}
