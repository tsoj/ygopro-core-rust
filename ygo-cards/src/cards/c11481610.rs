//! Performapal Popperup
use super::prelude::*;

pub const ID: u32 = 11481610;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES, SET_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn costchk(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsPlayerCanDraw(tp,#sg)
    (duel::is_player_can_draw(d, tp, sg.len() as u32), false)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, a.e);
    let g = duel::get_matching_group(d, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, h.map_or(NO_EX, |c| c.into()));
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return aux.SelectUnselectGroup(g,e,tp,1,3,s.costchk,0)
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(3), Some(&costchk), 0, a.tp, None, None, None, false), Err(true));
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,3,s.costchk,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(3), Some(&costchk), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // local ct=Duel.SendtoGrave(sg,REASON_COST)
    let ct = duel::send_to_grave(d, sg, REASON_COST, None, None);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ls: u32, rs: u32) -> bool {
    // ((c:IsSetCard(SET_MAGICIAN) and c:IsType(TYPE_PENDULUM)) or c:IsSetCard(SET_ODD_EYES) or c:IsSetCard(SET_PERFORMAPAL))
    // and c:GetLevel()>ls and c:GetLevel()<rs and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    ((card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_type(d, c, TYPE_PENDULUM))
        || card::is_set_card(d, c, &[SET_ODD_EYES as u16])
        || card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]))
        && card::get_level(d, c) > ls
        && card::get_level(d, c) < rs
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if Duel.Draw(tp,ct,REASON_EFFECT)==0 then return end
    if duel::draw(d, a.tp, ct as u32, REASON_EFFECT) == 0 {
        return;
    }
    // local summoned=false
    let mut summoned = false;
    // local pc1=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    let pc1 = duel::get_field_card(d, a.tp, LOCATION_PZONE, 0);
    // local pc2=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    let pc2 = duel::get_field_card(d, a.tp, LOCATION_PZONE, 1);
    // if pc1 and pc2 and pc1:IsFaceup() and pc2:IsFaceup() then
    if let (Some(pc1), Some(pc2)) = (pc1, pc2) {
        if card::is_faceup(d, pc1) && card::is_faceup(d, pc2) {
            // local ls,rs=pc1:GetLeftScale(),pc2:GetRightScale()
            let mut ls = card::get_left_scale(d, pc1);
            let mut rs = card::get_right_scale(d, pc2);
            // if ls>rs then ls,rs=rs,ls end
            if ls > rs {
                std::mem::swap(&mut ls, &mut rs);
            }
            // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp,ls,rs)
            let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ls, rs)), a.tp, LOCATION_HAND, 0, NO_EX);
            // local lc=Duel.GetLocationCount(tp,LOCATION_MZONE)
            let lc = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
            // if #g>0 and lc>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty() && lc > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
                // local sg=aux.SelectUnselectGroup(g,e,tp,1,math.min(ct,lc),aux.dncheck,1,tp,HINTMSG_SPSUMMON)
                let dn = aux::dncheck();
                let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(std::cmp::min(ct, lc as i64) as usize), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                // summoned=true
                summoned = true;
            }
        }
    }
    // if not summoned then
    if !summoned {
        // local lp=Duel.GetLP(tp)-(1000*Duel.GetFieldGroupCount(tp,LOCATION_HAND,0))
        let lp = duel::get_lp(d, a.tp) - 1000 * duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // Duel.SetLP(tp,math.max(lp,0))
        duel::set_lp(d, a.tp, lp.max(0));
    }
}
