//! A.I.'s Ritual
//! Ａｉの儀式
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 85327820;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Ritual.CreateProc(c,RITPROC_GREATER,aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE),nil,nil,s.extrafil,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_IGNISTER))
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_race(d, x, RACE_CYBERSE as u64))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extrafil(d, a))),
            matfilter: Some(ritual::MatFilt::F(Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8| card::is_set_card(d, x, &[SET_IGNISTER as u16])))),
            ..Default::default()
        },
    );
    // local tg=e1:GetTarget()
    let tg = effect::get_target(d, e1).nil();
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,...)
    effect::set_target(
        d,
        e1,
        Some(LuaFn::new(move |d, a| {
            let x = EffArgs::with_chk(a);
            // if chk==0 then
            if x.chk == Some(0) {
                // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_IGNISTER),tp,LOCATION_MZONE,0,1,nil) then
                if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_IGNISTER as u16]))), x.tp, LOCATION_MZONE, 0, 1, NO_EX) {
                    // e:SetLabel(1)
                    effect::set_label(d, x.e, &[1]);
                } else {
                    // e:SetLabel(0)
                    effect::set_label(d, x.e, &[0]);
                }
            }
            // if chk==1 and e:GetLabel()==1 then
            if x.chk == Some(1) && effect::get_label(d, x.e) == 1 {
                // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
                duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, x.tp, LOCATION_GRAVE as i32);
            }
            // return tg(e,tp,eg,ep,ev,re,r,rp,chk,...)
            tg.call(d, a)
        })),
    );
    // local op=e1:GetOperation()
    let op = effect::get_operation(d, e1).nil();
    // e1:SetOperation(function(e,...)
    effect::set_operation(
        d,
        e1,
        Some(LuaFn::new(move |d, a| {
            let x = EffArgs::new(a);
            // local ret=op(e,...)
            let ret = op.call(d, a);
            // if e:GetLabel()==1 then e:SetLabel(0) end
            if effect::get_label(d, x.e) == 1 {
                effect::set_label(d, x.e, &[0]);
            }
            // return ret
            ret
        })),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(c:GetControler(),CARD_SPIRIT_ELIMINATION)
    //     and c:IsSetCard(SET_IGNISTER) and c:IsLevelAbove(1) and c:IsAbleToRemove()
    duel::is_player_affected_by_effect(d, card::get_controler(d, c), CARD_SPIRIT_ELIMINATION).is_none()
        && card::is_set_card(d, c, &[SET_IGNISTER as u16])
        && card::is_level_above(d, c, 1)
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.extrafil(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrafil(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // return Duel.GetMatchingGroup(s.mfilter,tp,LOCATION_GRAVE,0,nil)
        return Some(duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_GRAVE, 0, NO_EX));
    }
    None
}
